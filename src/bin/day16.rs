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

    let (version_counter, packet_length) = consume_packet(iter.by_ref());
    dbg!(packet_length);
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

fn consume_packet(mut data: impl Iterator<Item = u8>) -> (u64, u64) {
    let mut packet_length = 0;
    let mut version_counter = 0;

    let version = read_3_bits(data.by_ref());
    packet_length += 3;
    dbg!(version);
    version_counter += version;

    let type_id = read_3_bits(data.by_ref());
    packet_length += 3;
    dbg!(type_id);

    match type_id {
        LITERAL_VALUE_TYPE_ID => {
            let (literal_value, literal_length) = consume_literal_value(data.by_ref());
            packet_length += literal_length;
        }
        _ => {
            let length_type_id = get_n_bits(data.by_ref(), 1)[0];
            packet_length += 1;
            if length_type_id == 0 {
                // the next 15 bits = total length in bits of the sub-packets contained by this packet
                let bits = get_n_bits(data.by_ref(), 15);
                packet_length += 15;
                let total_length_of_subpackets = bits_to_u64(&bits);
                dbg!(total_length_of_subpackets);
                let mut length_of_subpackets_so_far = 0;
                while total_length_of_subpackets > length_of_subpackets_so_far {
                    let (subpacket_version_counter, subpacket_length) =
                        consume_packet(data.by_ref());
                    version_counter += subpacket_version_counter;
                    length_of_subpackets_so_far += subpacket_length;
                }
            } else {
                // the next 11 bits = number that represents the number of sub-packets immediately contained by this packet
            }
        }
    }

    (version_counter, packet_length)
}

fn consume_literal_value(mut data: impl Iterator<Item = u8>) -> (u64, u64) {
    let mut literal_length = 0;
    let mut result: Vec<u8> = Vec::new();
    loop {
        let not_last_group = get_n_bits(data.by_ref(), 1)[0] == 1;
        let mut bits = get_n_bits(data.by_ref(), 4);
        literal_length += 5;
        result.append(&mut bits);

        if !not_last_group {
            break;
        }
    }

    let literal_value = bits_to_u64(&result);
    dbg!(literal_value);

    (literal_value, literal_length)
}

fn get_n_bits(data: impl Iterator<Item = u8>, n: usize) -> Vec<u8> {
    let bits: Vec<u8> = data.into_iter().take(n).collect();
    assert_eq!(bits.len(), n);

    bits
}
