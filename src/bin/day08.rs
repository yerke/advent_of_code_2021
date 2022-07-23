use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day08.txt")?);

    let mut counter_part1 = 0;

    for line in file.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split(" | ").collect();
        let right_part = parts[1];

        let segments: Vec<&str> = right_part.split_whitespace().collect();

        let unique_lengths: [usize; 4] = [2, 4, 3, 7];

        for segment in segments {
            if unique_lengths.contains(&segment.len()) {
                counter_part1 += 1;
            }
        }
    }

    println!("Part 1: {}", counter_part1);

    Ok(())
}
