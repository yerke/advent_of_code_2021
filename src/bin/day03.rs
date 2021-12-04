use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day03.txt")?);

    let mut number_of_lines = 0;
    let mut bits: Vec<u16> = Vec::new();
    for line in file.lines() {
        let line = line?;
        number_of_lines += 1;

        if bits.is_empty() {
            bits = vec![0; line.len()];
        }

        for (idx, b) in line.chars().enumerate() {
            match b {
                '0' => (),
                '1' => bits[idx] += 1,
                _ => panic!("Unexpected char"),
            }
        }
    }

    println!("Sum of bits: {:?}", bits);
    println!("Number of lines: {}", number_of_lines);

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for (_idx, sum) in bits.iter().enumerate() {
        let mut bit = 0;
        if *sum >= number_of_lines / 2 {
            bit = 1;
        }

        gamma = gamma * 2 + bit;
        epsilon = epsilon * 2 + (bit ^ 1);
    }

    println!(
        "gamma: {}, epsilon: {}, product: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    Ok(())
}
