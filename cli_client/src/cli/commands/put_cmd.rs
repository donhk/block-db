use crate::network::file_transfer_client::FileTransferClient;
use crate::network::MessageRequest;
use crate::utils::app_state::client_state;
use crate::utils::file_utils::{get_num_chunks, is_file_readable, read_file_chunks};

fn send_data(bytes: Vec<u8>, server_url: &String) {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let mut client = FileTransferClient::connect(server_url.to_string()).await.unwrap();
        let request = tonic::Request::new(
            MessageRequest {
                payload: bytes,
            }
        );
        let response = client.send_message(request).await.unwrap();
        println!("RESPONSE={:?}", response);
    });
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
    // use 1MB chunk size
    let chunk_size: usize = 1024 * 1024;
    let number_of_chunks = get_num_chunks(file_location, chunk_size);
    let server_url = client_state::get_client_conn().as_ref().unwrap();
    for chunk_index in 0..number_of_chunks {
        let chunk = read_file_chunks(file_location, chunk_size, chunk_index);
        if chunk.is_err() {
            println!("Error reading chunk {}", chunk_index);
            return;
        }
        send_data(chunk.unwrap(), server_url);
    }
}

