use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LITERAL_VALUE_TYPE_ID: u64 = 4;

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day16.txt")?);

    let mut first_line: String = String::new();
    for line in file.lines() {
        let line = line?;
        first_line = line;
        break;
    }

    let mut bits = Vec::new();
    for &hex in first_line.as_bytes() {
        let mut hex_bits = hex_to_bits(hex);
        bits.append(&mut hex_bits);
    }

    println!("{:?}", &bits);

    let mut iter = bits.into_iter();

    let version_counter = consume_packet(iter.by_ref());
    println!("Part 1: sum of versions: {}", version_counter);

    Ok(())
}

fn hex_to_bits(hex: u8) -> Vec<u8> {
    match hex {
        b'0' => vec![0, 0, 0, 0],
        b'1' => vec![0, 0, 0, 1],
        b'2' => vec![0, 0, 1, 0],
        b'3' => vec![0, 0, 1, 1],
        b'4' => vec![0, 1, 0, 0],
        b'5' => vec![0, 1, 0, 1],
        b'6' => vec![0, 1, 1, 0],
        b'7' => vec![0, 1, 1, 1],
        b'8' => vec![1, 0, 0, 0],
        b'9' => vec![1, 0, 0, 1],
        b'A' => vec![1, 0, 1, 0],
        b'B' => vec![1, 0, 1, 1],
        b'C' => vec![1, 1, 0, 0],
        b'D' => vec![1, 1, 0, 1],
        b'E' => vec![1, 1, 1, 0],
        b'F' => vec![1, 1, 1, 1],
        _ => panic!("Unexpected hex char"),
    }
}

fn read_3_bits(mut data: impl Iterator<Item = u8>) -> u64 {
    let bits: Vec<u8> = get_n_bits(data.by_ref(), 3);
    bits_to_u64(&bits)
}

fn bits_to_u64(data: &[u8]) -> u64 {
    let mut result = 0;
    for &e in data {
        result = result * 2 + e as u64;
    }

    result
}

fn consume_packet(mut data: impl Iterator<Item = u8>) -> u64 {
    let mut version_counter = 0;

    let version = read_3_bits(data.by_ref());
    dbg!(version);
    version_counter += version;

    let type_id = read_3_bits(data.by_ref());
    dbg!(type_id);

    match type_id {
        LITERAL_VALUE_TYPE_ID => {
            let result = consume_literal_value(data.by_ref());
        }
        _ => todo!(),
    }

    version_counter
}

fn consume_literal_value(mut data: impl Iterator<Item = u8>) -> u64 {
    let mut result: Vec<u8> = Vec::new();
    loop {
        let not_last_group = get_n_bits(data.by_ref(), 1)[0] == 1;
        let mut bits = get_n_bits(data.by_ref(), 4);
        result.append(&mut bits);

        if !not_last_group {
            break;
        }
    }

    dbg!(bits_to_u64(&result))
}

fn get_n_bits(data: impl Iterator<Item = u8>, n: usize) -> Vec<u8> {
    let bits: Vec<u8> = data.into_iter().take(n).collect();
    assert_eq!(bits.len(), n);

    bits
}
