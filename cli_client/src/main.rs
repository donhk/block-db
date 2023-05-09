mod term {
    pub mod terminal;
}

mod cli {
    pub mod cmds;
    pub mod cmd;

    pub(crate) mod commands {
        pub mod help_cmd;
        pub mod connect;
        pub mod status_cmd;
        pub mod cd;
        pub mod put;
        pub mod get_cmd;
        pub mod list_cmd;
    }
}

mod utils {
    pub mod app_state;
    pub mod file_utils;
}

use crate::term::terminal::start_terminal;
use crate::term::terminal::init_app;

pub mod network {
    tonic::include_proto!("network");
}

pub mod db {
    pub mod files_db;
}

fn main() {
    init_app();
    start_terminal();
}
