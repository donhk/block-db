use colored::Colorize;
use crate::db::files_db::{Document, documents_db};
use crate::network::file_transfer_client::FileTransferClient;
use crate::network::{MessageRequest, MessageResponse};
use crate::utils::app_state::client_state;
use crate::utils::file_utils::{get_num_chunks, hash_file, is_file_readable, read_file_chunks};
use std::time::{SystemTime};

use reed_solomon_erasure::galois_8::ReedSolomon;

pub fn upload_file(raw_cmd: &str) {
    let cmd_parts = raw_cmd.split_whitespace().collect::<Vec<&str>>();
    if cmd_parts.get(1).is_none() {
        println!("Provide a directory!");
        return;
    }
    if client_state::get_client_conn().is_none() {
        println!("Connect to a server first!");
        return;
    }
    let file_location = cmd_parts.get(1).unwrap();
    if !is_file_readable(file_location) {
        println!("Provide a readable file!");
        return;
    }

    //start process
    let start = SystemTime::now();
    // use 1MB chunk size
    let chunk_size: usize = 1024 * 1024;
    let number_of_chunks = get_num_chunks(file_location, chunk_size);
    let parity = std::cmp::max(2, number_of_chunks / 3);
    //define shards and parity shards
    let r = ReedSolomon::new(number_of_chunks, parity).unwrap();
    //master copy
    let mut master_copy: Vec<Vec<u8>> = Vec::new();
    let mut all_bytes_read: Vec<usize> = Vec::new();
    //read data to encode it
    for chunk_index in 0..number_of_chunks {
        let (chunk, bytes_read) = read_file_chunks(file_location, chunk_size, chunk_index);
        if chunk.is_err() {
            println!("Error reading chunk {}", chunk_index);
            return;
        }
        master_copy.push(chunk.unwrap());
        all_bytes_read.push(bytes_read);
    }
    for _ in 0..parity {
        master_copy.push(vec![0; chunk_size]);
        all_bytes_read.push(0);
    }
    for ma in all_bytes_read.iter() {
        println!("len: {}", ma)
    }
    // Construct the parity shards
    r.encode(&mut master_copy).unwrap();

    let server_url = client_state::get_client_conn().as_ref().unwrap();
    let mut message_ids: Vec<String> = Vec::new();
    for chunk in master_copy {
        let result = send_data(chunk, server_url);
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            tokio::select! {
            val = result => {
                message_ids.push(val.message_id);
            }
        }
        });
    }
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .unwrap_or_else(|e| e.duration());
    let hash = hash_file(file_location).unwrap();
    documents_db::insert(Document::new(
        file_location.to_string(),
        hash,
        message_ids,
        all_bytes_read,
        parity,
    ));
    println!("{} {}ms", "Saved".blue(), duration.as_millis().to_string().purple());
}

async fn send_data(bytes: Vec<u8>, server_url: &String) -> MessageResponse {
    let mut client = FileTransferClient::connect(server_url.to_string()).await.unwrap();
    let request = tonic::Request::new(
        MessageRequest {
            payload: bytes,
        }
    );
    let output = client.send_message(request).await;
    output.unwrap().into_inner()
}

