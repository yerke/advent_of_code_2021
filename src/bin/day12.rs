use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    println!("{:?}", &edges);
    println!("{:?}", &is_big);

    let mut visited_small = HashSet::new();
    let path_counter = advance("start", &edges, &is_big, &mut visited_small);

    println!("Part 1: {}", path_counter);

    Ok(())
}

fn advance(
    node: &str,
    edges: &HashMap<String, Vec<String>>,
    is_big: &HashSet<String>,
    visited_small: &mut HashSet<String>,
) -> u16 {
    if node == "end" {
        return 1;
    }

    if !is_big.contains(node) && visited_small.contains(node) {
        // cannot visit small node twice
        return 0;
    }

    if !is_big.contains(node) {
        visited_small.insert(node.into());
    }

    // iterate
    let mut counter = 0;
    for neighbor in edges.get(node).unwrap() {
        counter += advance(neighbor, edges, is_big, visited_small);
    }

    if !is_big.contains(node) {
        visited_small.remove(node.into());
    }

    counter
}
