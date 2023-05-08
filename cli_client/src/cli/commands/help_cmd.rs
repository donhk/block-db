use colored::Colorize;
use strum::IntoEnumIterator;
use crate::cli::cmds::Cmd;

pub fn print_help() {
    for cmd in Cmd::iter() {
        let help = cmd.help();
        println!("{} {}", cmd.emoji(), help.green());
    }
}