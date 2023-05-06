#[allow(dead_code)]
pub mod client_state {
    use tonic::transport::Channel;
    use crate::cli::commands::connect_cmd::network::file_transfer_client::FileTransferClient;

    // private static mutable variable
    static mut CLIENT_CONN: Option<FileTransferClient<Channel>> = None;
    static mut LOCATION: Option<String> = None;

    pub fn get_client_conn() -> &'static mut Option<FileTransferClient<Channel>> {
        unsafe {
            // use lazy initialization to create the instance if it doesn't exist yet
            return &mut CLIENT_CONN;
        }
    }

    pub fn set_client_conn(client_conn: FileTransferClient<Channel>) {
        unsafe {
            // use lazy initialization to create the instance if it doesn't exist yet
            let _ = CLIENT_CONN.insert(client_conn);
        }
    }

    pub fn get_location() -> &'static mut Option<String> {
        unsafe {
            // use lazy initialization to create the instance if it doesn't exist yet
            return &mut LOCATION;
        }
    }

    pub fn set_location(location: String) {
        unsafe {
            // use lazy initialization to create the instance if it doesn't exist yet
            let _ = LOCATION.insert(location);
        }
    }
}