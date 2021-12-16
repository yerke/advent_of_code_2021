use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let mut iter = bits.iter();

    let version = read_3_bits(iter.by_ref());
    dbg!(version);

    let type_id = read_3_bits(iter.by_ref());
    dbg!(type_id);

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

fn read_3_bits<'a>(data: impl Iterator<Item = &'a u8>) -> u8 {
    let bits: Vec<u8> = data.take(3).cloned().collect();
    assert_eq!(bits.len(), 3);

    bits_to_u8(&bits)
}

fn bits_to_u8(data: &[u8]) -> u8 {
    let mut result = 0;
    for &e in data {
        result = result * 2 + e;
    }

    result
}
