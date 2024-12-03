#[macro_use]
extern crate lazy_static;

use std::{env, path::PathBuf, str::FromStr};


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

pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            numbers
        })
        .collect()
}

pub fn valid_report(report: &[usize], increasing: bool) -> Result<bool, usize> {
    for (i, valid) in report
        .windows(2)
        .map(|w| is_valid(w[0], w[1], increasing))
        .enumerate()
    {
        if valid {
            continue;
        }
        // Either the current index, or the next index is invalid, to determine which:
        // If i and i + 2 is valid, then it's i+1 that's invalid
        let Some(j) = report.get(i + 2) else {
            return Err(i + 1);
        };
        if is_valid(report[i], *j, increasing) {
            return Err(i + 1);
        }
        return Err(i);
    }
    Ok(true)
}

fn is_valid(a: usize, b: usize, increasing: bool) -> bool {
    let res = (a > b) ^ increasing;
    match a.abs_diff(b) {
        1..=3 => res,
        _ => false,
    }
}