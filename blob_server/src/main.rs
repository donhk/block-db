use std::env;
use crate::service::file_transfer::start_server;

pub mod network {
    tonic::include_proto!("network");
}

mod service {
    pub mod file_transfer;
}

mod fs {
    pub mod io;
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let workspace = env::current_dir().unwrap().display().to_string();
    let storage = workspace + "/.storage";
    std::fs::create_dir_all(storage.as_str())?;
    println!("listening on: http://{}", addr);
    println!("storage dir:  {}", storage);
    let future = start_server(addr, storage);
    tokio::select! {
     _ = future => {}
    }
    Ok(())
}
