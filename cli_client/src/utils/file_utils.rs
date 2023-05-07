use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use sha2::{Sha256, Digest};

pub fn is_file_readable(filename: &str) -> bool {
    let path = Path::new(filename);
    if !path.exists() {
        println!("Path {} does not exist", filename);
        return false;
    }
    if !path.is_file() {
        println!("File {} does not exist", filename);
        return false;
    }
    if let Ok(_file) = File::open(path) {
        // File was successfully opened, so it can be read
        true
    } else {
        println!("File {} cannot be read", filename);
        // File could not be opened, so it cannot be read
        false
    }
}

pub fn read_file_chunks(filepath: &str, chunk_size: usize, chunk_index: usize) -> (io::Result<Vec<u8>>, usize) {
    let mut file = File::open(filepath).expect("Error opening file");
    let file_size = file.metadata().expect("Cannot get file size").len();
    let num_chunks = (file_size as usize + chunk_size - 1) / chunk_size;

    if chunk_index >= num_chunks {
        return (Err(io::Error::new(io::ErrorKind::InvalidInput, "chunk index out of range")), 0);
    }

    let mut chunk = vec![0u8; chunk_size];
    let start_pos = (chunk_index * chunk_size) as u64;
    file.seek(SeekFrom::Start(start_pos)).expect("Error during seek");
    let bytes_read = file.read(&mut chunk).expect("Cannot read");
    return (Ok(chunk), bytes_read);
}

pub fn get_num_chunks(file_path: &str, chunk_size: usize) -> usize {
    let file_size = fs::metadata(file_path).unwrap().len();
    (file_size as usize + chunk_size - 1) / chunk_size
}

pub fn hash_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024]; // Read the file in chunks of 1kb

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}