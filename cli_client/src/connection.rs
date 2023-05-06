use tokio::runtime::Runtime;
use tonic::transport::Channel;
use network::file_transfer_client::FileTransferClient;
//use crate::client_state::client_state;

pub mod network {
    tonic::include_proto!("network");
}

///
/// Initializes the connection to a given server
/// # Arguments
/// * `url` a url to connect
pub fn connect(url: &str) -> Option<FileTransferClient<Channel>> {
    let url_parts = url.split(' ').collect::<Vec<&str>>();
    if url_parts.get(1).is_none() {
        println!("Provide a server!");
        return None;
    }
    let server = url_parts.get(1).unwrap().to_string();
    println!("Connecting to server '{}'", server);
    let rt = Runtime::new().unwrap();
    let client = FileTransferClient::connect(server);
    let result = rt.block_on(client);
    if result.is_err() {
        return None;
    }
    return Some(result.unwrap());
}