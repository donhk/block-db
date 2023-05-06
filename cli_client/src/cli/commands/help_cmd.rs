use colored::Colorize;
use strum::IntoEnumIterator;
use crate::cli::cmds::Cmd;

pub fn print_help() {
    for cmd in Cmd::iter() {
        let cmd_str = cmd.to_string();
        println!("{} {}", cmd.emoji(), cmd_str.green());
    }
}