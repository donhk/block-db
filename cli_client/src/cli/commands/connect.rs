pub mod vader_cmds {
    use tokio::runtime::Runtime;
    use crate::utils::app_state::client_state;
    use colored::*;
    use crate::cli::cmd::CmdTrait;
    use crate::network::file_transfer_client::FileTransferClient;

    pub struct ConnectCmd {}

    impl CmdTrait for ConnectCmd {
        fn execute(&self, raw_cmd: &str) {
            self.connect(raw_cmd);
        }
    }

    impl ConnectCmd {
        pub fn new() -> ConnectCmd {
            ConnectCmd {}
        }

        ///
        /// Initializes the connection to a given server
        /// # Arguments
        /// * `url` a url to connect
        fn connect(&self, url: &str) {
            let url_parts = url.split_whitespace().collect::<Vec<&str>>();
            if url_parts.get(1).is_none() {
                println!("Provide a server!");
                return;
            }
            let server = url_parts.get(1).unwrap();
            println!("{} '{}'", "Connecting to server".blue(), server);
            let conn_req = FileTransferClient::connect(server.to_string());
            let result = Runtime::new().unwrap().block_on(conn_req);
            if result.is_err() {
                println!("{} '{}'", "Problem connecting to".red(), server);
                return;
            }
            println!("{}{}", "Connected!".green(), emojis::get("🐕").unwrap());
            client_state::set_client_conn(server.to_string());
        }
    }
}