use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day11.txt")?);

    let mut matrix: Vec<u8> = Vec::new();
    let mut width: usize = 0;
    let height: usize;

    for line in file.lines() {
        let line = line?;

        let mut row: Vec<u8> = line.as_bytes().iter().map(|&e| e - b'0').collect();
        width = row.len();
        matrix.append(&mut row);
    }

    height = matrix.len() / width;

    let diffs: [(i16, i16); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    let mut number_of_flashes = 0;

    let steps = 100;
    for step in 0..steps {
        // just increment all
        for i in 0..matrix.len() {
            matrix[i] += 1;
        }

        // check for flashes
        let mut flashed: Vec<bool> = vec![false; matrix.len()];
        let mut got_flash = false;

        loop {
            for i in 0..matrix.len() {
                if flashed[i] {
                    continue;
                }

                if matrix[i] < 10 {
                    continue;
                }

                got_flash = true;
                flashed[i] = true;
                number_of_flashes += 1;

                let (x, y) = ((i % width) as i16, (i / height) as i16);
                for diff in diffs {
                    let (x1, y1) = (x + diff.0, y + diff.1);
                    if are_valid_coordinates(width, height, x1, y1)
                        && !flashed[(y1 * (width as i16) + x1) as usize]
                    {
                        matrix[(y1 * (width as i16) + x1) as usize] += 1;
                    }
                }
            }

            if !got_flash {
                for i in 0..matrix.len() {
                    if flashed[i] {
                        matrix[i] = 0;
                    }
                }
                break;
            }

            got_flash = false;
        }

        // println!("step: {}", step);
        // pretty_print(&matrix, width, height);
    }

    println!("Part 1: {}", number_of_flashes);

    Ok(())
}

fn are_valid_coordinates(width: usize, height: usize, x: i16, y: i16) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    if x > (width as i16) - 1 || y > (height as i16) - 1 {
        return false;
    }
    true
}

fn pretty_print(matrix: &[u8], width: usize, height: usize) {
    for row in 0..height {
        for i in 0..width {
            print!("{}", (matrix[row * width + i] + b'0') as char);
        }
        println!();
    }
}
