use crate::app_state::client_state;
use colored::*;

pub fn print_status() {
    let conn_state = match client_state::get_client_conn().is_none() {
        true => "Yes".green(),
        false => "No".red(),
    };
    println!("Connected: {}", conn_state)
}