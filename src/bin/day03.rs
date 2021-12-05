use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    part1()?;
    part2()?;

    Ok(())
}

fn part1() -> Result<()> {
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

fn part2() -> Result<()> {
    let file = BufReader::new(File::open("data/day03.txt")?);

    let mut number_size = 0;
    let mut bit_size = 0;
    let mut numbers: Vec<u8> = Vec::new();

    for line in file.lines() {
        number_size += 1;
        let line = line?;

        bit_size = line.len();

        for b in line.chars() {
            match b {
                '0' => numbers.push(0),
                '1' => numbers.push(1),
                _ => panic!("Unexpected char"),
            }
        }
    }

    let oxygen = find_part2_value(&numbers, number_size, bit_size, true);
    let co2 = find_part2_value(&numbers, number_size, bit_size, false);
    println!(
        "Part 2: oxygen: {}, CO2: {}, product: {}",
        oxygen,
        co2,
        oxygen * co2
    );

    Ok(())
}

fn find_part2_value(
    numbers: &[u8],
    number_size: usize,
    bit_size: usize,
    prioritize_one: bool,
) -> u32 {
    let mut is_remaining_number: Vec<bool> = vec![true; number_size];

    for bit_idx in 0..bit_size {
        let mut number_of_remaining = 0;
        let mut sum_of_remaining_bits = 0;

        for i in 0..number_size {
            if is_remaining_number[i] {
                number_of_remaining += 1;
                sum_of_remaining_bits += numbers[bit_size * i + bit_idx] as u16;
            }
        }

        if number_of_remaining < 1 {
            panic!("Number of remaining number should not be less than 1");
        };

        if number_of_remaining == 1 {
            break;
        }

        let half = (number_of_remaining as f32) / 2.0;
        let lucky_bit = if sum_of_remaining_bits as f32 >= half {
            // only ones should remain
            if prioritize_one {
                1
            } else {
                0
            }
        } else {
            // only zeros should remain
            if prioritize_one {
                0
            } else {
                1
            }
        };

        for i in 0..number_size {
            if is_remaining_number[i] && numbers[bit_size * i + bit_idx] != lucky_bit {
                is_remaining_number[i] = false;
            }
        }
    }

    // Calculate
    let remaining_number_idx = is_remaining_number.iter().position(|e| *e).unwrap();
    let mut result = 0;
    for bit_idx in 0..bit_size {
        result = result * 2 + numbers[bit_size * remaining_number_idx + bit_idx] as u32;
    }

    result
}
