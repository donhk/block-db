use std::io::{self, Write};
use crate::cmds::Cmd;
use crate::help_cmd::print_help;
use colored::*;

pub fn start_terminal() {
    let bulb = emojis::get("💡").unwrap();
    let bomb = emojis::get("💣").unwrap();
    println!("{}{}", bulb.as_str(), "Welcome to Blob Vader".yellow());
    loop {
        print!("{} ", "blob>".cyan());
        io::stdout().flush().expect("Error flushing output buffer");
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("Error reading command");
        let cmd = cmd.trim();
        match cmd {
            s if s.to_lowercase().starts_with(Cmd::EXIT.to_string().as_str()) => {
                println!("Bye Bye!");
                break;
            }
            s if s.to_lowercase().starts_with(Cmd::CONNECT.to_string().as_str()) => {
                println!("Connecting to server");
            }
            s if s.to_lowercase().starts_with(Cmd::PUT.to_string().as_str()) => {
                println!("Uploading file");
            }
            s if s.to_lowercase().starts_with(Cmd::GET.to_string().as_str()) => {
                println!("Downloading file");
            }
            s if s.to_lowercase().starts_with(Cmd::LS.to_string().as_str()) => {
                println!("Listing file");
            }
            s if s.to_lowercase().starts_with(Cmd::HELP.to_string().as_str()) => {
                print_help();
            }
            _ => {
                println!("{} Unknown command '{}'", bomb.as_str(), cmd.red());
            }
        }
    }
}