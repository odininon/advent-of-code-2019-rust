use std::io::{BufReader, Error};
use std::fs::File;

pub fn read_file(path: &str) -> Result<BufReader<File>, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    Ok(buffered)
}
