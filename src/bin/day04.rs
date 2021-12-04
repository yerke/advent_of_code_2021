use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day04.txt")?);

    let mut lines = file.lines().into_iter();

    let line = lines.next().unwrap()?;
    let numbers: Vec<u8> = line
        .split(',')
        .map(|n| n.parse::<u8>().expect("Not a number"))
        .collect();

    let mut boards: Vec<Vec<(u8, bool)>> = Vec::new();
    for line in lines {
        let line = line?;

        if line.is_empty() {
            boards.push(Vec::new());
            continue;
        }

        let mut board_line: Vec<(u8, bool)> = line
            .split_whitespace()
            .map(|n| (n.parse::<u8>().expect("Not a number"), false))
            .collect();

        assert_eq!(board_line.len(), 5);

        boards.last_mut().unwrap().append(&mut board_line);
    }

    'outer: for n in numbers {
        for board in &mut boards {
            let idx = board.iter().position(|e| e == &(n, false));
            if idx.is_none() {
                continue;
            }

            board[idx.unwrap()] = (n, true);

            if is_bingo(board) {
                println!("{}", n);
                pretty_print(board);
                let unmarked_sum: u32 = board
                    .iter()
                    .filter_map(|(e, b)| match b {
                        true => None,
                        false => Some(*e as u32),
                    })
                    .sum();
                println!(
                    "Sum of unmarked: {}, product: {}",
                    unmarked_sum,
                    unmarked_sum * (n as u32)
                );
                break 'outer;
            }
        }
    }

    Ok(())
}

fn is_bingo(board: &[(u8, bool)]) -> bool {
    assert_eq!(board.len(), 25);

    for row in 0..5 {
        let mut found = true;

        for col in 0..5 {
            if board[row * 5 + col].1 == false {
                found = false;
                break;
            }
        }

        if found {
            return true;
        }
    }

    for col in 0..5 {
        let mut found = true;

        for row in 0..5 {
            if board[row * 5 + col].1 == false {
                found = false;
                break;
            }
        }

        if found {
            return true;
        }
    }

    false
}

fn pretty_print(board: &[(u8, bool)]) {
    for row in 0..5 {
        for col in 0..5 {
            print!("{:?}", board[row * 5 + col]);
        }
        println!();
    }
}
