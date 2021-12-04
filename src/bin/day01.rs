use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day01.txt")?);
    let mut previous = None;
    let mut counter = 0;

    for line in file.lines() {
        let line = line?;
        let depth: u32 = line.parse()?;
        if previous.is_some() {
            if depth > previous.unwrap() {
                counter += 1;
            }
        }
        previous = Some(depth);
    }

    println!("{}", counter);

    Ok(())
}
