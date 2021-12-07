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

    let mut lowest_cost = None;
    for p in min_position..max_position {
        let mut cost = 0;
        for e in &positions {
            cost += (e - p).abs();
        }

        // println!("p {}, cost {}", p, cost);

        if lowest_cost.is_none() {
            lowest_cost = Some(cost);
        }

        if lowest_cost.unwrap() > cost {
            lowest_cost = Some(cost);
        }
    }

    println!("Part 1: {}", lowest_cost.unwrap());

    Ok(())
}
