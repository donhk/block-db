use std::io::{self, Write};
use crate::cli::commands::help_cmd::print_help;
use crate::cli::commands::connect_cmd::connect;
use crate::cli::commands::status_cmd::print_status;
use crate::cli::commands::cd_cmd::change_dir;
use colored::*;
use crate::utils::app_state::client_state;
use std::env;
use crate::cli::cmds::Cmd;
use crate::cli::commands::get_cmd::download_file;
use crate::cli::commands::list_cmd::list;
use crate::cli::commands::put_cmd::upload_file;

pub fn init_app() {
    let workspace = env::current_dir().unwrap().display().to_string();
    client_state::set_location(workspace);
}

pub fn start_terminal() {
    let bulb = emojis::get("💡").unwrap();
    println!("{}{}", "Welcome to Blob Vader".yellow(), bulb.as_str());
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
                upload_file(&cmd);
            }
            s if s.starts_with(Cmd::GET.to_string().as_str()) => {
                download_file(&cmd);
            }
            s if s.starts_with(Cmd::LS.to_string().as_str()) => {
                list();
            }
            s if s.starts_with(Cmd::HELP.to_string().as_str()) => {
                print_help();
            }
            s if s.starts_with(Cmd::STATUS.to_string().as_str()) => {
                print_status();
            }
            s if s.starts_with(Cmd::CD.to_string().as_str()) => {
                change_dir(&cmd);
            }
            _ => {
                let bomb = emojis::get("💣").unwrap();
                println!("{} Unknown command '{}'", bomb.as_str(), cmd.red());
            }
        }
    }
}