use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day01.txt")?);
    let mut previous = None;
    let mut part1_counter = 0;
    let mut part2_counter = 0;
    let mut previous_arr: [Option<u32>; 3] = [None, None, None];

    for line in file.lines() {
        let line = line?;
        let depth: u32 = line.parse()?;
        if previous.is_some() {
            if depth > previous.unwrap() {
                part1_counter += 1;
            }
        }
        if previous_arr[0].is_some() && previous_arr[1].is_some() && previous_arr[2].is_some() {
            let previous_sum =
                previous_arr[0].unwrap() + previous_arr[1].unwrap() + previous_arr[2].unwrap();
            let new_sum = depth + previous_arr[0].unwrap() + previous_arr[1].unwrap();
            if new_sum > previous_sum {
                part2_counter += 1;
            }
        }

        previous = Some(depth);
        previous_arr[2] = previous_arr[1];
        previous_arr[1] = previous_arr[0];
        previous_arr[0] = previous;
    }

    println!("Part 1: {}", part1_counter);
    println!("Part 2: {}", part2_counter);

    Ok(())
}
