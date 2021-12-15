use anyhow::Result;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node(i16, i16, i64);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .2
            .cmp(&self.2)
            .then_with(|| self.0.cmp(&other.0))
            .then_with(|| self.1.cmp(&other.1))
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day15.txt")?);

    let mut maze: Vec<i16> = Vec::new();
    let mut width: usize = 0;
    let height: usize;
    for line in file.lines() {
        let line = line?;

        let mut row: Vec<i16> = line.as_bytes().iter().map(|&e| (e - b'0') as i16).collect();
        width = row.len();
        maze.append(&mut row);
    }
    height = maze.len() / width;

    // pretty_print(&maze, width, height);

    let lowest_cost = a_star(&maze, width, height, 0, 0);
    println!("Part 1: {}", &lowest_cost);

    let expanded_maze = expand_maze(&maze, width, height);
    // pretty_print(&expanded_maze, width * 5, height * 5);

    let lowest_cost = a_star(&expanded_maze, width * 5, height * 5, 0, 0);
    println!("Part 2: {}", &lowest_cost);

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

fn heuristic(width: usize, height: usize, x: i16, y: i16) -> i64 {
    (x as i64 - width as i64 - 1).abs() + (y as i64 - height as i64 - 1).abs()
}

fn a_star(maze: &[i16], width: usize, height: usize, start_x: i16, start_y: i16) -> i64 {
    let mut g_score: HashMap<(i16, i16), i64> = HashMap::new();
    let mut f_score: HashMap<(i16, i16), i64> = HashMap::new();
    let mut p_queue: BinaryHeap<Node> = BinaryHeap::new();
    g_score.insert((start_x, start_y), 0);
    f_score.insert(
        (start_x, start_y),
        0 + heuristic(width, height, start_x, start_y),
    );
    p_queue.push(Node(
        start_x,
        start_y,
        *f_score.get(&(start_x, start_y)).unwrap(),
    ));

    let diffs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut lowest_cost = 0;

    while !p_queue.is_empty() {
        let Node(x, y, _f) = p_queue.pop().unwrap();
        if x as usize == width - 1 && y as usize == height - 1 {
            lowest_cost = *g_score.get(&(x, y)).unwrap();
            break;
        }

        for diff in diffs {
            let (x1, y1) = (x + diff.0, y + diff.1);

            if !are_valid_coordinates(width, height, x1, y1) {
                continue;
            }

            let new_g =
                g_score.get(&(x, y)).unwrap() + maze[y1 as usize * width + x1 as usize] as i64;

            if new_g < *g_score.entry((x1, y1)).or_insert(i64::MAX) {
                g_score.insert((x1, y1), new_g);
                let new_f = new_g + heuristic(width, height, x1, y1);
                f_score.insert((x1, y1), new_f);
                p_queue.push(Node(x1, y1, new_f));
            }
        }
    }

    lowest_cost
}

fn expand_maze(maze: &[i16], width: usize, height: usize) -> Vec<i16> {
    let mut expanded_maze = vec![0; maze.len() * 25];

    // fill horizontally
    for (i, &e) in maze.iter().enumerate() {
        let mut e = e;
        let (x, y) = (i % width, i / width);
        for multiple in 0..5 {
            let (x1, y1) = (x + (multiple * width), y);
            // dbg!(x1, y1, e);
            expanded_maze[y1 * (width * 5) + x1] = e;
            if e == 9 {
                e = 1;
            } else {
                e += 1;
            }
        }
    }

    // fill vertically
    for multiple in 1..5 {
        for diff_y in 0..height {
            for diff_x in 0..width * 5 {
                let prev_idx =
                    (multiple - 1) * height * (width * 5) + diff_y * (width * 5) + diff_x;
                let idx = multiple * height * (width * 5) + diff_y * (width * 5) + diff_x;
                let mut e = expanded_maze[prev_idx];
                if e == 9 {
                    e = 1;
                } else {
                    e += 1;
                }
                expanded_maze[idx] = e;
            }
        }
    }

    expanded_maze
}

fn pretty_print(matrix: &[i16], width: usize, height: usize) {
    for row in 0..height {
        for i in 0..width {
            print!("{}", matrix[row * width + i]);
        }
        println!();
    }
}
