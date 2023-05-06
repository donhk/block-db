mod terminal;

mod cli {
    pub mod cmds;

    pub(crate) mod commands {
        pub mod help_cmd;
        pub mod connect_cmd;
        pub mod status_cmd;
        pub mod cd_cmd;
    }
}

mod app_state;

use terminal::start_terminal;
use crate::terminal::init_app;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_app();
    start_terminal();
    Ok(())
}
