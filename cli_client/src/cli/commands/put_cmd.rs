pub mod put {
    use colored::Colorize;
    use crate::db::files_db::{Document, documents_db};
    use crate::network::file_transfer_client::FileTransferClient;
    use crate::network::{MessageRequest, MessageResponse};
    use crate::utils::app_state::client_state;
    use crate::utils::file_utils::{get_num_chunks, hash_file, is_file_readable, read_file_chunks};
    use std::time::{SystemTime};
    use reed_solomon_erasure::galois_8::ReedSolomon;
    use crate::cli::cmd::CmdTrait;

    pub struct PutCmd {
        pub chunk_size: usize,
    }

    impl CmdTrait for PutCmd {
        fn execute(&self, raw_cmd: &str) {
            self.upload_file(raw_cmd);
        }
    }

    impl PutCmd {
        pub fn new() -> PutCmd {
            PutCmd {
                chunk_size: 1024 * 1024,
            }
        }

        fn validate_command<'a>(&self, raw_cmd: &'a str) -> (bool, &'a str) {
            let cmd_parts = raw_cmd.split_whitespace().collect::<Vec<&str>>();
            if cmd_parts.get(1).is_none() {
                println!("Provide a directory!");
                return (false, "");
            }
            if client_state::get_client_conn().is_none() {
                println!("Connect to a server first!");
                return (false, "");
            }
            let file_location = cmd_parts.get(1).unwrap();
            if !is_file_readable(file_location) {
                println!("Provide a readable file!");
                return (false, "");
            }
            return (true, file_location);
        }

        pub fn upload_file(&self, raw_cmd: &str) {
            // parse command
            let (valid, file_location) = self.validate_command(raw_cmd);
            if !valid {
                return;
            }
            //start process
            let start = SystemTime::now();
            let (valid, hash, message_ids, all_bytes_read, parity)
                = self.read_file_and_split(file_location);
            if !valid {
                return;
            }
            let end = SystemTime::now();
            let duration = end.duration_since(start)
                .unwrap_or_else(|e| e.duration());
            let document = Document::new(
                file_location.to_string(),
                hash,
                message_ids,
                all_bytes_read,
                parity,
            );
            documents_db::insert(document);
            println!("{} {}ms", "Saved".blue(), duration.as_millis().to_string().purple());
        }

        fn read_file_and_split(&self, file_location: &str) -> (bool, String, Vec<String>, Vec<usize>, usize) {
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
                    return (false, "".to_string(), Vec::default(), Vec::default(), 0);
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
                let result = self.send_data(chunk, server_url);
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    tokio::select! {
            val = result => {
                message_ids.push(val.message_id);
            }
        }
                });
            }
            let hash = hash_file(file_location).unwrap();
            return (true, hash, message_ids, all_bytes_read, parity);
        }

        async fn send_data(&self, bytes: Vec<u8>, server_url: &String) -> MessageResponse {
            let mut client = FileTransferClient::connect(server_url.to_string()).await.unwrap();
            let request = tonic::Request::new(
                MessageRequest {
                    payload: bytes,
                }
            );
            let output = client.send_message(request).await;
            output.unwrap().into_inner()
        }
    }
}
