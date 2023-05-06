use crate::utils::app_state::client_state;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use colored::*;

pub fn change_dir(raw_cmd: &str) {
    let cmd_parts = raw_cmd.split_whitespace().collect::<Vec<&str>>();
    if cmd_parts.get(1).is_none() {
        println!("Provide a directory!");
        return;
    }
    let location = cmd_parts.get(1).unwrap();
    let path = Path::new(location);
    if !is_writable_directory(path) {
        println!("{}.", "Directory is not writable".red());
    }
    client_state::set_location(location.to_string());
}

fn is_writable_directory(path: &Path) -> bool {
    let file_path = path.join("temp.txt");
    match File::create(&file_path) {
        Ok(mut file) => {
            let result = file.write(b"test");
            match result {
                Ok(_) => {
                    let _ = std::fs::remove_file(&file_path);
                    true
                }
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
