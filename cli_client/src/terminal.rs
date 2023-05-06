use std::io::{self, Write};
use crate::cmds::Cmd;
use crate::help_cmd::print_help;
use colored::*;
use crate::connection::connect;

pub fn start_terminal() {
    let bulb = emojis::get("💡").unwrap();
    println!("{}{}", bulb.as_str(), "Welcome to Blob Vader".yellow());
    loop {
        print!("{} ", "blob>".cyan());
        io::stdout().flush().expect("Error flushing output buffer");
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("Error reading command");
        let cmd = cmd.trim();
        let cmd_lower = cmd.to_lowercase();
        match cmd_lower {
            s if s.starts_with(Cmd::EXIT.to_string().as_str()) => {
                println!("Bye Bye!");
                break;
            }
            s if s.starts_with(Cmd::CONNECT.to_string().as_str()) => {
                connect(&cmd);
            }
            s if s.starts_with(Cmd::PUT.to_string().as_str()) => {
                println!("Uploading file");
            }
            s if s.starts_with(Cmd::GET.to_string().as_str()) => {
                println!("Downloading file");
            }
            s if s.starts_with(Cmd::LS.to_string().as_str()) => {
                println!("Listing file");
            }
            s if s.starts_with(Cmd::HELP.to_string().as_str()) => {
                print_help();
            }
            _ => {
                let bomb = emojis::get("💣").unwrap();
                println!("{} Unknown command '{}'", bomb.as_str(), cmd.red());
            }
        }
    }
}