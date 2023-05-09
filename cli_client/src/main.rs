mod term {
    pub mod terminal;
}

mod cli {
    pub mod cmds;
    pub mod cmd;

    pub(crate) mod commands {
        pub mod help;
        pub mod connect;
        pub mod status;
        pub mod cd;
        pub mod put;
        pub mod get;
        pub mod list;
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
