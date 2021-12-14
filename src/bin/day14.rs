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

    let number_of_steps = 40;
    let counts = glue_together(&template, &rules, number_of_steps);

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

fn count_elements(arr: &[u8], counts: &mut HashMap<u8, u64>) {
    for &e in arr {
        *counts.entry(e).or_insert(0) += 1;
    }
}

fn apply_rules(template: &[u8], rules: &HashMap<(u8, u8), u8>, number_of_steps: usize) -> Vec<u8> {
    let mut template: Vec<u8> = template.into();
    for _ in 0..number_of_steps {
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

    template
}

fn apply_with_memoization(
    (start, end): (u8, u8),
    rules: &HashMap<(u8, u8), u8>,
    lookup: &mut HashMap<(u8, u8), Vec<u8>>,
    number_of_steps: usize,
) -> Vec<u8> {
    let mut result = Vec::new();
    if let Some(cached) = lookup.get(&(start, end)) {
        result = cached.clone();
    } else {
        result = apply_rules(&[start, end], &rules, number_of_steps);
        result.pop();
        lookup.insert((start, end), result.clone());
    }

    result
}

fn glue_together(
    template: &[u8],
    rules: &HashMap<(u8, u8), u8>,
    number_of_steps: usize,
) -> HashMap<u8, u64> {
    let half = number_of_steps / 2;

    let mut lookup: HashMap<(u8, u8), Vec<u8>> = HashMap::new();
    let mut intermediate = Vec::new();
    let mut result = Vec::new();

    // First half
    let mut start = template[0];
    for &end in template.iter().skip(1) {
        result = apply_with_memoization((start, end), rules, &mut lookup, half);
        intermediate.append(&mut result);

        start = end;
    }

    intermediate.push(template[template.len() - 1]);

    println!("Got intermediate of size: {}", &intermediate.len());
    println!("Got lookup of size: {}", &lookup.len());

    // Second half
    let mut counts = HashMap::new();
    let mut start = intermediate[0];
    for &end in intermediate.iter().skip(1) {
        result = apply_with_memoization((start, end), rules, &mut lookup, half);
        count_elements(&result, &mut counts);

        start = end;
    }

    *counts
        .entry(intermediate[intermediate.len() - 1])
        .or_insert(0) += 1;

    counts
}
