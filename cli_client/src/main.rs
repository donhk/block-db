use crate::term::terminal::term::Terminal;

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
        pub mod unknown;
        pub mod net;
    }
}

mod utils {
    pub mod app_state;
    pub mod file_utils;
}

mod term {
    pub mod terminal;
}

pub mod network {
    tonic::include_proto!("network");
}

pub mod db {
    pub mod files_db;
}

fn main() {
    let terminal = Terminal::new();
    terminal.init_app();
    terminal.start_terminal();
}
