pub mod vader_cmds {
    use crate::utils::app_state::client_state;
    use colored::*;
    use crate::db::files_db::documents_db;

    use crate::cli::cmd::CmdTrait;

    pub struct StatusCmd {}

    impl StatusCmd {
        pub fn new() -> StatusCmd {
            StatusCmd {}
        }
    }

    impl CmdTrait for StatusCmd {
        fn execute(&self) {
            let conn_state = match client_state::get_client_conn().is_none() {
                true => "No".red(),
                false => "Yes".green(),
            };
            let workspace = match client_state::get_location().is_none() {
                true => "Unknown".red(),
                false => client_state::get_location().as_ref().unwrap().green(),
            };
            println!("Connected: {}", conn_state);
            println!("workspace: {}", workspace);
            if documents_db::get_map().is_none() {
                return;
            }
            for (k, v) in documents_db::get_map().unwrap().iter() {
                let key = k.clone();
                let value = v.clone();
                println!("parity: {} hash: {} name: {} chunks: {}", value.parity_chunks, value.hash, key, value.chunks.len())
            }
        }
    }
}