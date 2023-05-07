use colored::Colorize;
use crate::db::files_db::{Document, documents_db};
use crate::network::file_transfer_client::FileTransferClient;
use crate::network::{MessageRequest, MessageResponse};
use crate::utils::app_state::client_state;
use crate::utils::file_utils::{get_num_chunks, hash_file, is_file_readable, read_file_chunks};
use std::time::{SystemTime};

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
    let start = SystemTime::now();
    // use 1MB chunk size
    let chunk_size: usize = 1024 * 1024;
    let number_of_chunks = get_num_chunks(file_location, chunk_size);
    let server_url = client_state::get_client_conn().as_ref().unwrap();
    let mut message_ids: Vec<String> = Vec::new();
    for chunk_index in 0..number_of_chunks {
        let chunk = read_file_chunks(file_location, chunk_size, chunk_index);
        if chunk.is_err() {
            println!("Error reading chunk {}", chunk_index);
            return;
        }
        let result = send_data(chunk.unwrap(), server_url);
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
    ));
    println!("{} {}ms", "Saved".blue(), duration.as_millis().to_string().purple());
}

