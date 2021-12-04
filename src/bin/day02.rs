use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day02.txt")?);

    let mut x = 0;
    let mut y = 0;
    for line in file.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split(' ').collect();
        assert_eq!(parts.len(), 2);

        let command = parts[0];
        let units: u32 = parts[1].parse()?;

        match command {
            "forward" => x += units,
            "down" => y += units,
            "up" => y -= units,
            _ => panic!("Unexpected command: {}", command),
        }
    }

    println!("X: {}, Y: {}, product: {}", x, y, x * y);

    Ok(())
}
