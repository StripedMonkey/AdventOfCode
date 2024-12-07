#[macro_use]
extern crate lazy_static;

use itertools::{repeat_n, Itertools};
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


pub fn has_valid_perm(test_value: usize, numbers: &Vec<usize>, operations: &[fn(usize, usize) -> usize]) -> bool {

    repeat_n(operations.iter(), numbers.len() - 1)
        .multi_cartesian_product()
        .map(|operations| {
            let mut result = numbers[0];
            for (idx, operation) in operations.iter().enumerate() {
                result = operation(result, numbers[idx + 1]);
            }
            result
        })
        .any(|res| res == test_value)
}

pub fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (test_value, numbers) = line.split_once(": ").unwrap();
            let test_value = test_value.parse().unwrap();
            let numbers = numbers.split(" ").map(|n| n.parse().unwrap()).collect();
            (test_value, numbers)
        })
        .collect()
}
