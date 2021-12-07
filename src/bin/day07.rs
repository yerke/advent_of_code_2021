use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day07.txt")?);

    let positions: Vec<i32> = file
        .lines()
        .into_iter()
        .next()
        .unwrap()?
        .split(',')
        .map(|e| e.parse().unwrap())
        .collect();

    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();

    let mut lowest_cost1 = None;
    let mut lowest_cost2 = None;
    for p in min_position..max_position {
        let mut cost1 = 0;
        let mut cost2 = 0;

        for e in &positions {
            let diff = (e - p).abs();
            cost1 += diff;
            cost2 += (diff + 1) * diff / 2;
        }

        if lowest_cost1.is_none() {
            lowest_cost1 = Some(cost1);
            lowest_cost2 = Some(cost2);
        }

        if lowest_cost1.unwrap() > cost1 {
            lowest_cost1 = Some(cost1);
        }

        if lowest_cost2.unwrap() > cost2 {
            lowest_cost2 = Some(cost2);
        }
    }

    println!("Part 1: {}", lowest_cost1.unwrap());
    println!("Part 2: {}", lowest_cost2.unwrap());

    Ok(())
}
