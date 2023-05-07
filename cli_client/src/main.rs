mod term {
    pub mod terminal;
}

mod cli {
    pub mod cmds;

    pub(crate) mod commands {
        pub mod help_cmd;
        pub mod connect_cmd;
        pub mod status_cmd;
        pub mod cd_cmd;
        pub mod put_cmd;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_app();
    start_terminal();
    Ok(())
}
