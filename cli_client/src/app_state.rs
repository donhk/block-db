#[allow(dead_code)]
pub mod client_state {
    use tonic::transport::Channel;
    use crate::connect_cmd::network::file_transfer_client::FileTransferClient;

    // private static mutable variable
    static mut CLIENT_CONN: Option<FileTransferClient<Channel>> = None;

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
}