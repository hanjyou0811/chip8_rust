use std::{fs::File, io::{self, Read}, path::Path};

pub fn load(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let mut rom = File::open(path)?;
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)?;
    Ok(buffer)
}
