pub mod vader_cmds {
    use crate::utils::app_state::client_state;
    use crate::utils::file_utils::is_writable_directory;
    use std::path::Path;
    use colored::*;
    use crate::cli::cmd::CmdTrait;

    pub struct CdCmd {}

    impl CmdTrait for CdCmd {
        fn execute(&self, raw_cmd: &str) {
            self.change_dir(raw_cmd);
        }
    }

    impl CdCmd {
        pub fn new() -> CdCmd {
            CdCmd {}
        }
        fn change_dir(&self, raw_cmd: &str) {
            let cmd_parts = raw_cmd.split_whitespace().collect::<Vec<&str>>();
            if cmd_parts.get(1).is_none() {
                println!("Provide a directory!");
                return;
            }
            let location = cmd_parts.get(1).unwrap();
            let path = Path::new(location);
            if !is_writable_directory(path) {
                println!("{}.", "Directory is not writable".red());
            }
            client_state::set_location(location.to_string());
        }
    }
}