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
            Cmd::CONNECT => write!(f, "connect"),
            Cmd::PUT => write!(f, "put"),
            Cmd::GET => write!(f, "get"),
            Cmd::HELP => write!(f, "help"),
            Cmd::STATUS => write!(f, "status"),
            Cmd::CD => write!(f, "cd"),
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

impl Cmd {
    pub fn help(&self) -> &str {
        match self {
            Cmd::LS =>      "ls             # display files in current work directory",
            Cmd::EXIT =>    "exit           # exits",
            Cmd::CONNECT => "connect <addr> # connects to server",
            Cmd::PUT =>     "put <file>     # sends file to server",
            Cmd::GET =>     "get <file> <dest> # gets a file from the server and writes to dest",
            Cmd::HELP =>    "help           # shows this message",
            Cmd::STATUS =>  "status         # shows client metadata",
            Cmd::CD =>      "cd <directory> # changes the work directory",
        }
    }
}