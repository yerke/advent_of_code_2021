use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day05.txt")?);
    let mut present: HashMap<(u16, u16), u16> = HashMap::new();

    for line in file.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split(" -> ").collect();
        let start_parts: Vec<u16> = parts[0].split(',').map(|e| e.parse().unwrap()).collect();
        let end_parts: Vec<u16> = parts[1].split(',').map(|e| e.parse().unwrap()).collect();

        let x1 = start_parts[0];
        let y1 = start_parts[1];
        let x2 = end_parts[0];
        let y2 = end_parts[1];

        if x1 == x2 {
            // Handle vertical line
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            for y in min_y..=max_y {
                let old_value = *present.get(&(x1, y)).or(Some(&0)).unwrap();
                present.insert((x1, y), old_value + 1);
            }
        } else if y1 == y2 {
            // Handle horizontal line
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            for x in min_x..=max_x {
                let old_value = *present.get(&(x, y1)).or(Some(&0)).unwrap();
                present.insert((x, y1), old_value + 1);
            }
        }
    }

    // dbg!(&present);

    let mut overlapping_count = 0;
    for (_, counter) in present {
        if counter > 1 {
            overlapping_count += 1;
        }
    }

    println!("Part 1: {}", overlapping_count);

    Ok(())
}
