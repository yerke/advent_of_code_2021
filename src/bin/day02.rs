use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day02.txt")?);

    let mut x = 0;
    let mut y1 = 0;

    let mut aim = 0;
    let mut y2 = 0;

    for line in file.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split(' ').collect();
        assert_eq!(parts.len(), 2);

        let command = parts[0];
        let units: u32 = parts[1].parse()?;

        match command {
            "forward" => {
                x += units;
                y2 += aim * units;
            }
            "down" => {
                y1 += units;
                aim += units;
            }
            "up" => {
                y1 -= units;
                aim -= units;
            }
            _ => panic!("Unexpected command: {}", command),
        }
    }

    println!("Part 1: X: {}, Y: {}, product: {}", x, y1, x * y1);
    println!("Part 2: X: {}, Y: {}, product: {}", x, y2, x * y2);

    Ok(())
}
