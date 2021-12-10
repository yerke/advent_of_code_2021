use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day10.txt")?);

    let opening_chars = [b'(', b'[', b'{', b'<'];
    let closing_chars = [b')', b']', b'}', b'>'];
    let matching_chars = HashMap::from([(b'(', b')'), (b'[', b']'), (b'{', b'}'), (b'<', b'>')]);
    let points_map_part1: HashMap<u8, u32> =
        HashMap::from([(b')', 3), (b']', 57), (b'}', 1197), (b'>', 25137)]);
    let points_map_part2: HashMap<u8, u64> =
        HashMap::from([(b')', 1), (b']', 2), (b'}', 3), (b'>', 4)]);

    let mut part1_points = 0;
    let mut part2_line_points: Vec<u64> = Vec::new();

    for line in file.lines() {
        let line = line?;

        let mut is_corrupted = false;
        let mut stack: Vec<u8> = Vec::new();
        for &c in line.as_bytes() {
            if opening_chars.contains(&c) {
                stack.push(*matching_chars.get(&c).unwrap());
                continue;
            }

            if !closing_chars.contains(&c) {
                panic!("Unexpected char: {}", c);
            }

            if stack.is_empty() || stack.pop().unwrap() != c {
                part1_points += points_map_part1.get(&c).unwrap();
                is_corrupted = true;
            }
        }

        if !is_corrupted && !stack.is_empty() {
            let mut line_points: u64 = 0;
            stack.reverse();
            for c in stack.iter() {
                line_points = line_points * 5 + points_map_part2.get(c).unwrap();
            }

            part2_line_points.push(line_points);
        }
    }

    part2_line_points.sort();
    let middle_idx = part2_line_points.len() / 2;

    println!("Part 1 points: {}", part1_points);
    println!("Part 2 points: {}", part2_line_points[middle_idx]);

    Ok(())
}
