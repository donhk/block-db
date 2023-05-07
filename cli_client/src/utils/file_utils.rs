use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

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

pub fn read_file_chunks(filepath: &str, chunk_size: usize, chunk_index: usize) -> io::Result<Vec<u8>> {
    let mut file = File::open(filepath)?;
    let file_size = file.metadata()?.len();
    let num_chunks = (file_size as usize + chunk_size - 1) / chunk_size;

    if chunk_index >= num_chunks {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "chunk index out of range"));
    }

    let mut chunk = vec![0u8; chunk_size];
    let start_pos = (chunk_index * chunk_size) as u64;
    file.seek(SeekFrom::Start(start_pos))?;
    let bytes_read = file.read(&mut chunk)?;
    if bytes_read < chunk_size {
        chunk.resize(bytes_read, 0);
    }
    return Ok(chunk);
}

pub fn get_num_chunks(file_path: &str, chunk_size: usize) -> usize {
    let file_size = fs::metadata(file_path).unwrap().len();
    (file_size as usize + chunk_size - 1) / chunk_size
}