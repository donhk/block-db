use tonic::{transport::Server, Request, Response, Status};

use network::file_transfer_server::{FileTransfer, FileTransferServer};
use network::{MessageRequest, MessageResponse};
use uuid::Uuid;

pub mod network {
    tonic::include_proto!("network");
}

#[derive(Debug, Default)]
pub struct FileTransferService {}

#[tonic::async_trait]
impl FileTransfer for FileTransferService {
    async fn send_message(&self, request: Request<MessageRequest>)
                          -> Result<Response<MessageResponse>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        println!("{}", String::from_utf8(req.payload).unwrap());
        let reply = MessageResponse {
            successful: true,
            message_id: Uuid::new_v4().to_string(),
        };

        return Ok(Response::new(reply));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:3000".parse()?;
    println!("listening on: http://{}", addr);
    let transfer_service = FileTransferService::default();
    Server::builder().add_service(FileTransferServer::new(transfer_service))
        .serve(addr)
        .await?;

    Ok(())
}
