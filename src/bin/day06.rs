use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day06.txt")?);
    let line = file.lines().into_iter().next().unwrap()?;

    let fish_arr: Vec<u8> = line.split(',').map(|e| e.parse::<u8>().unwrap()).collect();
    let mut fish: HashMap<u8, u64> = HashMap::new();

    for f in 0..=8 {
        fish.insert(f, *&fish_arr.iter().filter(|e| **e == f).count() as u64);
    }

    let days = 256;

    for d in 0..days {
        let mut next_generation = HashMap::new();

        for (f, count) in fish {
            if f == 0 {
                *next_generation.entry(6).or_insert(0) += count;
                *next_generation.entry(8).or_insert(0) += count;
            } else {
                *next_generation.entry(f - 1).or_insert(0) += count;
            }
        }

        fish = next_generation;

        if d == 79 {
            println!("Part 1: {}", &fish.values().sum::<u64>());
        }
    }

    println!("Part 2: {}", fish.values().sum::<u64>());

    Ok(())
}
