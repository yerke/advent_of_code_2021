use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day06.txt")?);
    let line = file.lines().into_iter().next().unwrap()?;

    let mut fish: Vec<u16> = line.split(',').map(|e| e.parse::<u16>().unwrap()).collect();

    let days = 80;

    for d in 0..days {
        let mut next_generation = Vec::new();

        for f in fish {
            if f == 0 {
                next_generation.push(6);
                next_generation.push(8);
            } else {
                next_generation.push(f - 1);
            }
        }

        fish = next_generation;

        // println!("{:?}", &fish);
    }

    println!("Part 1: {}", fish.len());

    Ok(())
}
