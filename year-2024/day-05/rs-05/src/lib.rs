#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use std::{cmp::Ordering, env, path::PathBuf, str::FromStr};
// The input texts are static, should it be? Probably not, but it was an excuse to do it this way.
lazy_static! {
    pub static ref INPUT_1: &'static str = static_read("input1.txt");
}

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
pub fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let mut ordering_rules = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let Some((left, right)) = line.split_once("|") else {
            unreachable!("Invalid");
        };
        let left = left.parse::<usize>().unwrap();
        let right = right.parse::<usize>().unwrap();
        ordering_rules.push((left, right));
    }
    ordering_rules.sort();
    let page_updates: Vec<Vec<usize>> = lines
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    (ordering_rules, page_updates)
}

pub fn compare(
    ordering_rules: &Vec<(usize, usize)>,
) -> impl FnMut(&usize, &usize) -> Ordering + '_ {
    |a, b| {
        if ordering_rules.binary_search(&(*a, *b)).is_ok() {
            Ordering::Greater
        } else if ordering_rules.binary_search(&(*b, *a)).is_ok() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

pub fn is_valid(update: &Vec<usize>, ordering_rules: &Vec<(usize, usize)>) -> bool {
    let mut comparator = compare(&ordering_rules);
    update
        .iter()
        .tuple_windows()
        .all(|(a, b)| comparator(a, b) != Ordering::Less)
}
