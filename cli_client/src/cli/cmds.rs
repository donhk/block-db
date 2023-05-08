use std::{fmt};

use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Cmd {
    LS,
    EXIT,
    CONNECT,
    PUT,
    GET,
    HELP,
    STATUS,
    CD,
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cmd::LS => write!(f, "ls"),
            Cmd::EXIT => write!(f, "exit"),
            Cmd::CONNECT => write!(f, "connect <addr>"),
            Cmd::PUT => write!(f, "put <file_name>"),
            Cmd::GET => write!(f, "get <file_name> <output_location>"),
            Cmd::HELP => write!(f, "help"),
            Cmd::STATUS => write!(f, "status"),
            Cmd::CD => write!(f, "cd <location>"),
        }
    }
}

impl Cmd {
    pub fn emoji(&self) -> &str {
        match self {
            Cmd::LS => emojis::get("🗺️").unwrap().as_str(),
            Cmd::EXIT => emojis::get("🕳️").unwrap().as_str(),
            Cmd::CONNECT => emojis::get("🕸️").unwrap().as_str(),
            Cmd::PUT => emojis::get("🦄").unwrap().as_str(),
            Cmd::GET => emojis::get("🐖").unwrap().as_str(),
            Cmd::HELP => emojis::get("🤪").unwrap().as_str(),
            Cmd::STATUS => emojis::get("🤓").unwrap().as_str(),
            Cmd::CD => emojis::get("🍇").unwrap().as_str(),
        }
    }
}