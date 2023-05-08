use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;
use crate::fs::io::{read_file_bytes, write_bytes_to_file};
use crate::network::file_transfer_server::{FileTransfer, FileTransferServer};
use crate::network::{MessageRequest, MessageResponse, MessageReadRequest, MessageReadResponse};

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
    async fn send_message(&self, request: Request<MessageRequest>)
                          -> Result<Response<MessageResponse>, Status> {
        println!("message received");
        let req = request.into_inner();
        let uuid = Uuid::new_v4();
        let file = self.storage.as_str().to_owned() + "/" + uuid.to_string().as_str();
        let result = write_bytes_to_file(&req.payload, file.as_str());
        let reply = MessageResponse {
            successful: result.is_ok(),
            message_id: uuid.to_string(),
        };

        return Ok(Response::new(reply));
    }

    async fn receive_message(&self, request: Request<MessageReadRequest>)
                             -> Result<Response<MessageReadResponse>, Status> {
        println!("message sent");
        let req = request.into_inner();
        let id = req.message_id;
        let file = self.storage.as_str().to_owned() + "/" + id.as_str();
        let payload = read_file_bytes(file.as_str());
        let reply = MessageReadResponse {
            successful: payload.is_ok(),
            payload: payload.unwrap_or_default(),
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