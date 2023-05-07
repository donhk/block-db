use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;
use crate::fs::io::write_bytes_to_file;
use crate::network::file_transfer_server::{FileTransfer, FileTransferServer};
use crate::network::{MessageRequest, MessageResponse};

#[derive(Debug, Default)]
pub struct FileTransferService {
    storage: String,
}

impl FileTransferService {
    pub fn new(storage: String) -> Self {
        FileTransferService { storage }
    }
}

#[tonic::async_trait]
impl FileTransfer for FileTransferService {
    async fn send_message(&self, request: Request<MessageRequest>) -> Result<Response<MessageResponse>, Status> {
        println!("Got a request {:?}", request.metadata());
        let req = request.into_inner();
        println!("{} bytes", req.payload.len());
        let uuid = Uuid::new_v4();
        let file = self.storage.as_str().to_owned() + "/" + uuid.to_string().as_str();
        let result = write_bytes_to_file(&req.payload, file.as_str());
        let reply = MessageResponse {
            successful: result.is_ok(),
            message_id: uuid.to_string(),
        };

        return Ok(Response::new(reply));
    }
}

pub async fn start_server(addr: SocketAddr, workspace: String) -> Result<(), Box<dyn std::error::Error>> {
    let transfer_service = FileTransferService::new(workspace);
    Server::builder().add_service(FileTransferServer::new(transfer_service))
        .serve(addr)
        .await?;
    Ok(())
}