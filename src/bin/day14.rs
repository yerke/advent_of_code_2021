use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = BufReader::new(File::open("data/day14.txt")?);

    let mut template: Vec<u8> = Vec::new();
    let mut rules: HashMap<(u8, u8), u8> = HashMap::new();

    for line in file.lines() {
        let line = line?;

        if template.is_empty() {
            // TODO: Clean up
            template.reserve(line.as_bytes().len());
            for &b in line.as_bytes() {
                template.push(b);
            }
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(" -> ").collect();
        let (start, end) = (parts[0].as_bytes()[0], parts[0].as_bytes()[1]);
        rules.insert((start, end), parts[1].as_bytes()[0]);
    }

    pretty_print(&template);
    // println!("{:?}", &rules);

    let number_of_steps = 10;

    for step in 0..number_of_steps {
        let mut result = Vec::with_capacity(template.len() * 2);
        let mut start = template[0];
        for &end in template.iter().skip(1) {
            result.push(start);
            if let Some(&in_between) = rules.get(&(start, end)) {
                result.push(in_between);
            }

            start = end;
        }

        result.push(template[template.len() - 1]);

        template = result;
    }

    pretty_print(&template);

    let counts = count_elements(&template);

    println!("{:?}", &counts);

    let &(&min_e, &min_count) = &counts
        .iter()
        .min_by(|&(&a, &b), &(&c, &d)| b.cmp(&d))
        .unwrap();

    let &(&max_e, &max_count) = &counts
        .iter()
        .min_by(|&(&a, &b), &(&c, &d)| d.cmp(&b))
        .unwrap();

    println!("Part 1: {}", max_count - min_count);

    Ok(())
}

fn pretty_print(arr: &[u8]) {
    for &e in arr {
        print!("{}", e as char);
    }
    println!();
}

fn count_elements(arr: &[u8]) -> HashMap<u8, u32> {
    let mut counts = HashMap::new();
    for &e in arr {
        *counts.entry(e).or_insert(0) += 1;
    }
    counts
}
