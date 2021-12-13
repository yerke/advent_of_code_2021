use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day13.txt")?);

    let mut paper = HashSet::new();
    let mut fold_instructions: Vec<(String, u16)> = Vec::new();

    for line in file.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        if line.starts_with("fold") {
            // TODO
            let (_, parts) = line.rsplit_once(' ').unwrap();
            let (coordinate, value) = parts.split_once('=').unwrap();
            fold_instructions.push((coordinate.into(), value.parse::<u16>().unwrap()));
            continue;
        }

        let parts: Vec<u16> = line.split(',').map(|n| n.parse().unwrap()).collect();
        let (x, y) = (parts[0], parts[1]);
        paper.insert((x, y));
    }

    for (idx, instruction) in fold_instructions.iter().enumerate() {
        paper = fold(&paper, instruction);

        if idx == 0 {
            println!("Parts 1: number of dots after 1 fold: {}", paper.len());
        }
    }

    println!("Part 2 answer:");
    pretty_print(&paper);

    Ok(())
}

fn fold(paper: &HashSet<(u16, u16)>, instruction: &(String, u16)) -> HashSet<(u16, u16)> {
    let coordinate = &instruction.0;
    let fold_value = instruction.1;
    let mut new_paper: HashSet<(u16, u16)> = HashSet::new();

    for (x, y) in paper.iter() {
        let (mut x, mut y) = (*x, *y);

        if coordinate == "x" {
            if x > fold_value {
                x = x - (x - fold_value) * 2;
            }
        } else {
            if y > fold_value {
                y = y - (y - fold_value) * 2;
            }
        }

        new_paper.insert((x, y));
    }

    new_paper
}

fn pretty_print(paper: &HashSet<(u16, u16)>) {
    let max_x = *&paper.iter().fold(0, |max_x, (x, _y)| max_x.max(*x));
    let max_y = *&paper.iter().fold(0, |max_y, (_x, y)| max_y.max(*y));

    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&(x, y)) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
