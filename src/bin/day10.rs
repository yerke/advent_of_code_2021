use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day10.txt")?);

    let opening_chars = [b'(', b'[', b'{', b'<'];
    let closing_chars = [b')', b']', b'}', b'>'];
    let matching_chars = HashMap::from([(b'(', b')'), (b'[', b']'), (b'{', b'}'), (b'<', b'>')]);
    let points_map: HashMap<u8, u32> =
        HashMap::from([(b')', 3), (b']', 57), (b'}', 1197), (b'>', 25137)]);

    let mut line_mismatches = Vec::new();
    let mut part1_points = 0;

    for line in file.lines() {
        let line = line?;

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
                line_mismatches.push(c);
                part1_points += points_map.get(&c).unwrap();
            }
        }
    }

    println!("Part 1 points: {}", part1_points);

    Ok(())
}
