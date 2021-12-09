use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day09.txt")?);

    let mut map: Vec<i16> = Vec::new();
    let mut width: i16 = 0;
    let height: i16;

    for line in file.lines() {
        let line = line?;

        let mut row: Vec<i16> = line.as_bytes().iter().map(|b| (b - b'0') as i16).collect();
        width = row.len() as i16;

        map.append(&mut row);
    }

    height = (map.len() / width as usize) as i16;

    let diffs = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    let mut low_point_indices = Vec::new();
    let mut part1_result = 0;

    for (idx, val) in map.iter().enumerate() {
        let x = idx as i16 % width;
        let y = idx as i16 / width;
        let mut found = true;

        for diff in diffs {
            let x1 = x + diff.0;
            let y1 = y + diff.1;
            if are_valid_coordinates(width, height, x1, y1)
                && &map[(y1 * width + x1) as usize] <= val
            {
                found = false;
                break;
            }
        }

        if found {
            low_point_indices.push(idx);
            part1_result += val + 1;
        }
    }

    println!("Part 1: {}", part1_result);

    Ok(())
}

fn are_valid_coordinates(width: i16, height: i16, x: i16, y: i16) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    if x > width - 1 || y > height - 1 {
        return false;
    }
    true
}
