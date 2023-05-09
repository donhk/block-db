use colored::*;
use crate::utils::app_state::client_state;
use std::env;
use rustyline::{DefaultEditor};
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use crate::cli::cmd::CmdTrait;
use crate::cli::cmds::Cmd;
use crate::cli::commands::cd::vader_cmds::CdCmd;
use crate::cli::commands::connect::vader_cmds::ConnectCmd;
use crate::cli::commands::put::vader_cmds::PutCmd;
use crate::cli::commands::help::vader_cmds::PrintCmd;
use crate::cli::commands::list::vader_cmds::ListCmd;
use crate::cli::commands::status::vader_cmds::StatusCmd;
use crate::cli::commands::get::vader_cmds::GetCmd;

pub fn init_app() {
    let workspace = env::current_dir().unwrap().display().to_string();
    client_state::set_location(workspace);
}

pub fn start_terminal() {
    let bulb = emojis::get("💡").unwrap();
    println!("{} {}", "Welcome to Document-Vader".yellow(), bulb.as_str());
    let mut rl = DefaultEditor::new().unwrap();
    rl.set_max_history_size(100).expect("Error setting history size");
    loop {
        let readline = rl.readline(&*"blob>".cyan().to_string());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("error saving history");
                let cmd = line.trim();
                if cmd.is_empty() {
                    continue;
                }
                let cmd_lower = cmd.to_lowercase();
                if cmd_lower == Cmd::EXIT.to_string().as_str() {
                    println!("Bye Bye!");
                    break;
                }
                match cmd_lower {
                    s if s.starts_with(Cmd::CONNECT.to_string().as_str()) => {
                        let connect = ConnectCmd::new(&cmd);
                        connect.execute();
                    }
                    s if s.starts_with(Cmd::PUT.to_string().as_str()) => {
                        let put = PutCmd::new(&cmd);
                        put.execute();
                    }
                    s if s.starts_with(Cmd::GET.to_string().as_str()) => {
                        let get = GetCmd::new(&cmd);
                        get.execute();
                    }
                    s if s.starts_with(Cmd::LS.to_string().as_str()) => {
                        let list = ListCmd::new();
                        list.execute();
                    }
                    s if s.starts_with(Cmd::HELP.to_string().as_str()) => {
                        let help = PrintCmd::new();
                        help.execute();
                    }
                    s if s.starts_with(Cmd::STATUS.to_string().as_str()) => {
                        let status = StatusCmd::new();
                        status.execute();
                    }
                    s if s.starts_with(Cmd::CD.to_string().as_str()) => {
                        let cd = CdCmd::new(&cmd);
                        cd.execute();
                    }
                    _ => {
                        let bomb = emojis::get("💣").unwrap();
                        println!("{} Unknown command '{}'", bomb.as_str(), cmd.red());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C, use exit command to get out");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}