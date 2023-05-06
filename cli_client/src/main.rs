pub mod payments {
    tonic::include_proto!("network");
}

mod terminal;
mod help_cmd;
mod cmds;
mod connection;
mod app_state;

use terminal::start_terminal;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_terminal();

    Ok(())
}
