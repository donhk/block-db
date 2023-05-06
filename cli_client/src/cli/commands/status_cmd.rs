use crate::utils::app_state::client_state;
use colored::*;

pub fn print_status() {
    let conn_state = match client_state::get_client_conn().is_none() {
        true => "No".red(),
        false => "Yes".green(),
    };
    let workspace = match client_state::get_location().is_none() {
        true => "Unknown".red(),
        false => client_state::get_location().as_ref().unwrap().green(),
    };
    println!("Connected: {}", conn_state);
    println!("workspace: {}", workspace);
}