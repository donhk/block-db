pub mod vader_cmds {
    use std::fs;
    use crate::utils::app_state::client_state;
    use chrono::{DateTime, Local};
    use prettytable::{Table, row};
    use colored::*;
    use crate::cli::cmd::CmdTrait;

    pub struct ListCmd {}

    impl ListCmd {
        pub fn new() -> ListCmd {
            ListCmd {}
        }
    }

    impl CmdTrait for ListCmd {
        fn execute(&self, raw_cmd: &str) {
            // Create the table
            let mut table = Table::new();
            // Add a row per time
            let file = emojis::get("📄").unwrap().as_str();
            let time = emojis::get("⏲️").unwrap().as_str();
            table.add_row(row![
        "file_name".to_owned().green().to_string()+file,
        "created".to_owned().green().to_string()+time
    ]);
            let workspace = client_state::get_location().as_ref().unwrap();
            let current_dir = fs::read_dir(workspace).unwrap();
            for entry in current_dir {
                let entry = entry.unwrap();
                let file_name = entry.file_name().to_string_lossy().to_string();
                let created = entry.metadata().unwrap().created().unwrap();
                let datetime: DateTime<Local> = created.into();
                let time_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                table.add_row(row![file_name, time_str]);
            }

            // Print the table to stdout
            table.printstd();
        }
    }
}