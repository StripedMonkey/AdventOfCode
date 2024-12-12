use std::{collections::HashMap, iter, path::PathBuf, str::FromStr};

use itertools::Itertools as _;

// Read a file path relative to the parent of the current day's project directory.
// Each day contains the inputs and examples for that day in the the parent, so I can
// (in theory/later) share the inputs between multiple languages. Pretend like I'll actually do that.
pub fn static_read(file_path: &str) -> &'static str {
    let mut cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).expect("Infallible action failed!");
    cwd.pop();
    let file_path = cwd.join(file_path);
    println!("{:?}", file_path);
    let file = std::fs::read_to_string(file_path).expect("Failed to open file!");
    Box::leak(file.into_boxed_str())
}

pub fn rule_1(value: usize) -> Option<Box<dyn Iterator<Item = usize>>> {
    if value == 0 {
        return Some(Box::new(iter::once(1)));
    }
    None
}

pub fn rule_2(value: usize) -> Option<Box<dyn Iterator<Item = usize>>> {
    let digits = value.ilog10() + 1;
    if digits % 2 == 0 {
        let upper = value / 10usize.pow(digits / 2);
        let lower = value % 10usize.pow(digits / 2);
        return Some(Box::new(iter::once(upper).chain(iter::once(lower))));
    }
    None
}

pub fn rule_3(value: usize) -> Option<Box<dyn Iterator<Item = usize>>> {
    Some(Box::new(iter::once(value * 2024)))
}

pub fn step(values: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let rules = [rule_1, rule_2, rule_3];
    let mut new = HashMap::new();

    for (value, count) in values.iter() {
        for rule in rules.iter() {
            let Some(new_values) = rule(*value) else {
                continue;
            };
            for new_value in new_values {
                new.insert(new_value, new.get(&new_value).unwrap_or(&0) + count);
            }
            break;
        }
    }
    new
}

pub fn parse(input: &str) -> HashMap<usize, usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .fold(HashMap::new(), |mut acc, x: usize | {
            acc.insert(x, acc.get(&x).unwrap_or(&0) + 1);
            acc
        })
}
