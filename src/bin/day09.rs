use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
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
        let (x, y) = index_to_coordinates(idx, width, height);
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

    let mut basin_sizes = Vec::new();

    for idx in low_point_indices.iter() {
        let idx = *idx;
        let mut basin_size = 0;
        let mut seen: HashSet<usize> = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(idx);
        seen.insert(idx);

        while !queue.is_empty() {
            let next_idx = queue.pop_front().unwrap();
            basin_size += 1;
            let (x, y) = index_to_coordinates(next_idx, width, height);

            for diff in diffs {
                let x1 = x + diff.0;
                let y1 = y + diff.1;
                if are_valid_coordinates(width, height, x1, y1)
                    && !seen.contains(&((y1 * width + x1) as usize))
                    && *&map[(y1 * width + x1) as usize] != 9
                {
                    queue.push_back((y1 * width + x1) as usize);
                    seen.insert((y1 * width + x1) as usize);
                }
            }
        }

        basin_sizes.push(basin_size);
    }

    basin_sizes.sort();

    println!("Part 1: {}", part1_result);
    println!(
        "Part 2: {:?}",
        basin_sizes[basin_sizes.len() - 1]
            * basin_sizes[basin_sizes.len() - 2]
            * basin_sizes[basin_sizes.len() - 3]
    );

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

fn index_to_coordinates(idx: usize, width: i16, height: i16) -> (i16, i16) {
    let x = idx as i16 % width;
    let y = idx as i16 / width;
    (x, y)
}
