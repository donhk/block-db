use std::fs::File;
use std::io::Write;

pub fn write_bytes_to_file(bytes: &[u8], filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(bytes)?;
    Ok(())
}