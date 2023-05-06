pub mod payments {
    tonic::include_proto!("network");
}

mod terminal;
mod help_cmd;
mod cmds;
mod connect_cmd;
mod app_state;
mod status_cmd;
mod cd_cmd;

use terminal::start_terminal;
use crate::terminal::init_app;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_app();
    start_terminal();
    Ok(())
}
