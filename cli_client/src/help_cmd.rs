use strum::IntoEnumIterator;
use crate::cmds::Cmd;

pub fn print_help() {
    for cmd in Cmd::iter() {
        println!("{:?}", cmd);
    }
}