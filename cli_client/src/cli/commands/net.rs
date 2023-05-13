pub mod vader_cmds {
    use std::time::SystemTime;
    use crate::cli::cmd::CmdTrait;
    use colored::*;
    use rand::Rng;
    use crate::network::file_transfer_client::FileTransferClient;
    use crate::network::{UploadNetworkRequest, UploadNetworkResponse};
    use crate::utils::app_state::client_state;

    trait UploadTrait {
        fn upload(&self);
    }

    trait DownloadTrait {
        fn download(&self);
    }

    pub struct NetCmd {
        upload: bool,
    }

    impl NetCmd {
        pub fn new(raw_cmd: &str) -> Self {
            let cmd = raw_cmd.to_lowercase();
            if cmd.contains("up") {
                Self {
                    upload: true,
                }
            } else {
                Self {
                    upload: false,
                }
            }
        }

        async fn send_data(&self, bytes: Vec<u8>, server_url: &String) -> UploadNetworkResponse {
            let mut client = FileTransferClient::connect(server_url.to_string()).await.unwrap();
            let request = tonic::Request::new(
                UploadNetworkRequest {
                    payload: bytes,
                }
            );
            let output = client.network_load(request).await;
            output.unwrap().into_inner()
        }
    }

    impl CmdTrait for NetCmd {
        fn execute(&self) {
            if self.upload {
                self.upload();
            } else {
                self.download();
            }
        }
    }

    impl UploadTrait for NetCmd {
        fn upload(&self) {
            if client_state::get_client_conn().is_none() {
                println!("Connect to a server first!");
                return;
            }
            // Calculate the number of bytes for 1 MB
            let size_mb = 1;
            let size_bytes = size_mb * 1024 * 1024;
            // Generate the vector with random bytes
            let mut rng = rand::thread_rng();
            let server_url = client_state::get_client_conn().as_ref().unwrap();
            for i in 0..5 {
                let data: Vec<u8> = (0..size_bytes).map(|_| rng.gen::<u8>()).collect();
                let start = SystemTime::now();
                self.send_data(data, server_url);
            }
        }
    }

    impl DownloadTrait for NetCmd {
        fn download(&self) {
            if client_state::get_client_conn().is_none() {
                println!("Connect to a server first!");
                return;
            }
        }
    }
}