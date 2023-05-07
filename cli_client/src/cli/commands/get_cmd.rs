use std::fs::File;
use std::io::Write;
use colored::Colorize;
use crate::db::files_db::documents_db;
use crate::network::file_transfer_client::FileTransferClient;
use crate::network::{MessageReadRequest, MessageReadResponse};
use crate::utils::app_state::client_state;
use crate::utils::file_utils::hash_file;

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
    let doc = doc.unwrap();
    let mut file = File::create(target_file).unwrap();
    for chunk in doc.chunks {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let response = get_data(chunk.clone()).await;
            if !response.successful {
                println!("Failed chunk: {}", chunk);
                return;
            }
            file.write(&response.payload).expect("Error writing file");
        });
    }
    let new_hash = hash_file(target_file).unwrap();
    if doc.hash != new_hash {
        println!("{} original: {} new: {}", "The file is corrupted".red(), doc.hash, new_hash);
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