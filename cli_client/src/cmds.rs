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
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cmd::LS => write!(f, "ls"),
            Cmd::EXIT => write!(f, "exit"),
            Cmd::CONNECT => write!(f, "connect"),
            Cmd::PUT => write!(f, "put"),
            Cmd::GET => write!(f, "get"),
            Cmd::HELP => write!(f, "help"),
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
        }
    }
}