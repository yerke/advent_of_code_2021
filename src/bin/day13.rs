use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day13.txt")?);

    let mut paper = HashSet::new();
    let mut fold_instructions: Vec<(String, u16)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

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

        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    println!("paper: {:?}", &paper);
    println!("max_x: {}", max_x);
    println!("max_y: {}", max_y);
    println!("fold_instructions: {:?}", &fold_instructions);

    println!("Dots before: {}", paper.len());

    let new_paper = fold(&paper, &fold_instructions[0]);

    println!("Dots after: {}", new_paper.len());

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
