use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
enum Mode {
    Part1,
    Part2,
}

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day12.txt")?);

    let mut is_big: HashSet<String> = HashSet::new();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();

    for line in file.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split('-').collect();
        edges
            .entry(parts[0].into())
            .or_insert(Vec::new())
            .push(parts[1].into());

        edges
            .entry(parts[1].into())
            .or_insert(Vec::new())
            .push(parts[0].into());

        for part in parts {
            if part.chars().next().unwrap().is_ascii_uppercase() {
                is_big.insert(part.into());
            }
        }
    }

    let mut visited_small = HashMap::new();
    let path_counter_part1 = advance(
        "start",
        &edges,
        &is_big,
        &mut visited_small,
        &Mode::Part1,
        None,
    );
    let path_counter_part2 = advance(
        "start",
        &edges,
        &is_big,
        &mut visited_small,
        &Mode::Part2,
        None,
    );

    println!("Part 1: {}", path_counter_part1);
    println!("Part 2: {}", path_counter_part2);

    Ok(())
}

fn advance(
    node: &str,
    edges: &HashMap<String, Vec<String>>,
    is_big: &HashSet<String>,
    visited_small: &mut HashMap<String, u8>,
    mode: &Mode,
    single_small: Option<&str>,
) -> u32 {
    if node == "end" {
        return 1;
    }

    visited_small.entry(node.into()).or_insert(0);

    if node == "start" && *visited_small.get(node).unwrap() != 0 {
        return 0;
    }

    if mode == &Mode::Part1 && !is_big.contains(node) && *visited_small.get(node).unwrap() != 0 {
        // cannot visit small node twice in Part 1
        return 0;
    }

    if !is_big.contains(node) && *visited_small.get(node).unwrap() != 0 && single_small.is_some() {
        // cannot visit small node twice in Part 2, if we already visited some other small cave twice
        return 0;
    }

    let mut local_single_small = single_small;
    if !is_big.contains(node) && *visited_small.get(node).unwrap() != 0 {
        assert!(local_single_small.is_none());
        local_single_small = Some(node);
    }

    assert!(*visited_small.get_mut(node).unwrap() <= 2);

    if !is_big.contains(node) {
        *visited_small.get_mut(node).unwrap() += 1;
    }

    // iterate
    let mut counter = 0;
    for neighbor in edges.get(node).unwrap() {
        counter += advance(
            neighbor,
            edges,
            is_big,
            visited_small,
            mode,
            local_single_small,
        );
    }

    if !is_big.contains(node) {
        *visited_small.get_mut(node).unwrap() -= 1;
    }

    counter
}
