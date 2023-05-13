pub mod term {
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
    use crate::cli::commands::net::vader_cmds::NetCmd;
    use crate::cli::commands::unknown::vader_cmds::UnknownCmd;

    pub struct Terminal {}

    impl Terminal {
        pub fn new() -> Self {
            Terminal {}
        }

        pub fn init_app(&self) {
            let workspace = env::current_dir().unwrap().display().to_string();
            client_state::set_location(workspace);
        }

        pub fn start_terminal(&self) {
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
                        let command: Box<dyn CmdTrait> = self.get_command(&cmd);
                        command.execute();
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

        fn get_command<'a>(&self, cmd: &'a str) -> Box<dyn CmdTrait + 'a> {
            return if cmd.starts_with(Cmd::CONNECT.to_string().as_str()) {
                Box::new(ConnectCmd::new(&cmd))
            } else if cmd.starts_with(Cmd::PUT.to_string().as_str()) {
                Box::new(PutCmd::new(&cmd))
            } else if cmd.starts_with(Cmd::GET.to_string().as_str()) {
                Box::new(GetCmd::new(&cmd))
            } else if cmd.starts_with(Cmd::LS.to_string().as_str()) {
                Box::new(ListCmd::new())
            } else if cmd.starts_with(Cmd::HELP.to_string().as_str()) {
                Box::new(PrintCmd::new())
            } else if cmd.starts_with(Cmd::STATUS.to_string().as_str()) {
                Box::new(StatusCmd::new())
            } else if cmd.starts_with(Cmd::CD.to_string().as_str()) {
                Box::new(CdCmd::new(&cmd))
            }else if cmd.starts_with(Cmd::NET.to_string().as_str()) {
                Box::new(NetCmd::new(&cmd))
            }
            else {
                Box::new(UnknownCmd::new(&cmd))
            };
        }
    }
}