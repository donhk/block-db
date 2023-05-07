use crate::utils::app_state::client_state;
use colored::*;
use crate::db::files_db::documents_db;

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
    if documents_db::get_map().is_none() {
        return;
    }
    for (k, v) in documents_db::get_map().unwrap().iter() {
        let key = k.clone();
        let value = v.clone();
        println!("{}: {}", key, value.chunks.len())
    }
}