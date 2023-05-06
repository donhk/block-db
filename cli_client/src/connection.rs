use tokio::runtime::Runtime;
use network::file_transfer_client::FileTransferClient;
use crate::app_state::client_state;
use colored::*;

pub mod network {
    tonic::include_proto!("network");
}

///
/// Initializes the connection to a given server
/// # Arguments
/// * `url` a url to connect
pub fn connect(url: &str) {
    let url_parts = url.split_whitespace().collect::<Vec<&str>>();
    if url_parts.get(1).is_none() {
        println!("Provide a server!");
        return;
    }
    let server = url_parts.get(1).unwrap();
    println!("{} '{}'", "Connecting to server".blue(), server);
    let rt = Runtime::new().unwrap();
    let client = FileTransferClient::connect(server.to_string());
    let result = rt.block_on(client);
    if result.is_err() {
        println!("{} '{}'", "Problem connecting to".red(), server);
        return;
    }
    println!("{}{}", "Connected!".green(), emojis::get("🐕").unwrap());
    client_state::set_client_conn(result.unwrap());
}