use colored::Colorize;
use strum::IntoEnumIterator;
use crate::cmds::Cmd;

pub fn print_help() {
    let rocket = emojis::get("🚀").unwrap();
    for cmd in Cmd::iter() {
        let cmd_str = cmd.to_string();
        println!("{} {}", rocket.as_str(), cmd_str.green());
    }
}