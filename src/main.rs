use std::io::Error;

mod utilities;
mod day1;

fn main() -> Result<(), Error> {
    day1::solve("inputs/input-1.txt")?;

    Ok(())
}