pub mod vader_cmds {
    use tokio::runtime::Runtime;
    use crate::utils::app_state::client_state;
    use colored::*;
    use crate::cli::cmd::CmdTrait;
    use crate::network::file_transfer_client::FileTransferClient;

    // We've also added a private field _marker of type
    // std::marker::PhantomData<&'a ()> to tie the lifetime of 'a to the instance of ConnectCmd.
    // This is a common trick used to ensure that Rust's borrow checker understands the lifetime
    // relationships between types.
    pub struct ConnectCmd<'a> {
        raw_cmd: &'a str,
    }

    impl<'a> CmdTrait for ConnectCmd<'a> {
        fn execute(&self) {
            self.connect();
        }
    }

    impl<'a> ConnectCmd<'a> {
        pub fn new(raw_cmd: &'a str) -> Self {
            ConnectCmd {
                raw_cmd
            }
        }

        ///
        /// Initializes the connection to a given server
        /// # Arguments
        /// * `url` a url to connect
        fn connect(&self) {
            let url_parts = self.raw_cmd.split_whitespace().collect::<Vec<&str>>();
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