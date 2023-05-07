use std::fs::File;
use std::io::Write;
use colored::Colorize;
use crate::db::files_db::documents_db;
use crate::network::file_transfer_client::FileTransferClient;
use crate::network::{MessageReadRequest, MessageReadResponse};
use crate::utils::app_state::client_state;
use crate::utils::file_utils::hash_file;

use reed_solomon_erasure::galois_8::ReedSolomon;

pub fn download_file(raw_cmd: &str) {
    let cmd_parts = raw_cmd.split_whitespace().collect::<Vec<&str>>();
    if cmd_parts.len() == 1 {
        println!("{} {}", "Invalid command: ".red(), "get <filename> <target>".blue());
        return;
    }
    let source_file = *cmd_parts.get(1).unwrap();
    let target_file = *cmd_parts.get(2).unwrap();
    let doc = documents_db::get(source_file);
    if doc.is_none() {
        println!("{}", "Invalid document name '{}'".red());
        return;
    }
    let document = doc.unwrap();

    let parity = 2;
    let number_of_chunks = document.chunks.len() - parity;
    //define shards and parity shards
    let r = ReedSolomon::new(number_of_chunks, parity).unwrap();
    let mut shards: Vec<Option<Vec<u8>>> = Vec::new();
    for chunk_index in 0..document.chunks.len() {
        let chunk_id = document.chunks.get(chunk_index).unwrap();
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let response = get_data(chunk_id.clone()).await;
            if !response.successful {
                println!("Missing chunk: {}", chunk_index);
                shards.push(None);
            } else {
                shards.push(Some(response.payload));
            }
        });
    }

    r.reconstruct(&mut shards).unwrap();
    let result: Vec<_> = shards.clone().into_iter().filter_map(|x| x).collect();

    let mut file = File::create(target_file).unwrap();
    for id in 0..result.len() - (parity) {
        let shard = shards.get(id).unwrap().clone();
        let mut bytes = shard.unwrap();
        let num_of_bytes = document.bytes_read.get(id).unwrap();
        bytes.resize(*num_of_bytes, 0);
        println!("Getting {} size {} bytes", id, bytes.len());
        file.write(&bytes).expect("Error writing file");
    }
    let new_hash = hash_file(target_file).unwrap();
    if document.hash != new_hash {
        println!("{} original: {} new: {}", "The file is corrupted".red(), document.hash, new_hash);
    }
}

async fn get_data(id: String) -> MessageReadResponse {
    let server_url = client_state::get_client_conn().as_ref().unwrap();
    let mut client = FileTransferClient::connect(server_url.to_string()).await.unwrap();
    let request = tonic::Request::new(
        MessageReadRequest {
            message_id: id,
        }
    );
    let output = client.receive_message(request).await;
    output.unwrap().into_inner()
}