use anyhow::Result;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node(i8, i8, i64);

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

    let mut maze: Vec<i8> = Vec::new();
    let mut width: usize = 0;
    let height: usize;
    for line in file.lines() {
        let line = line?;

        let mut row: Vec<i8> = line.as_bytes().iter().map(|&e| (e - b'0') as i8).collect();
        width = row.len();
        maze.append(&mut row);
    }
    height = maze.len() / width;

    let mut g_score: HashMap<(i8, i8), i64> = HashMap::new();
    let mut f_score: HashMap<(i8, i8), i64> = HashMap::new();
    let mut p_queue: BinaryHeap<Node> = BinaryHeap::new();
    g_score.insert((0, 0), 0);
    f_score.insert((0, 0), 0 + heuristic(width, height, 0, 0));
    p_queue.push(Node(0, 0, *f_score.get(&(0, 0)).unwrap()));

    let diffs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut lowest_cost = 0;

    while !p_queue.is_empty() {
        let Node(x, y, f) = p_queue.pop().unwrap();
        // dbg!(x, y, f);
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
            // dbg!(x1, y1, new_g);

            if new_g < *g_score.entry((x1, y1)).or_insert(i64::MAX) {
                g_score.insert((x1, y1), new_g);
                let new_f = new_g + heuristic(width, height, x1, y1);
                // dbg!(x1, y1, new_g, new_f);
                f_score.insert((x1, y1), new_f);
                p_queue.push(Node(x1, y1, new_f));
            }
        }
    }

    println!("Part 1: {}", &lowest_cost);

    Ok(())
}

fn are_valid_coordinates(width: usize, height: usize, x: i8, y: i8) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    if x > (width as i8) - 1 || y > (height as i8) - 1 {
        return false;
    }
    true
}

fn heuristic(width: usize, height: usize, x: i8, y: i8) -> i64 {
    (x as i64 - width as i64 - 1).abs() + (y as i64 - height as i64 - 1).abs()
}
