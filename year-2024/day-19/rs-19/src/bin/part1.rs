use core::num;
use std::collections::HashSet;

use aoc_utils::*;
use rs_2024_19::*;

fn main() {
    let input = rs_2024_19::static_read("input1.txt");
    let (available_towels, desired_patterns) = parse(&input);
    let expr = available_towels.join("|");
    let expr = regex::Regex::new(&format!(r"^({})+$", expr)).unwrap();
    let possible_patterns = desired_patterns
        .iter()
        .filter(|pat| expr.is_match(pat))
        .count();
    println!("{:?}", possible_patterns);
}

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();
    let available_towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    lines.next();
    let desired_patterns = lines.map(|s| s.to_string()).collect();
    (available_towels, desired_patterns)
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_19::static_read("example1.txt");
        let (available_towels, desired_patterns) = parse(&input);
        let expr = available_towels.join("|");
        let expr = regex::Regex::new(&format!(r"^({})+$", expr)).unwrap();
        let possible_patterns = desired_patterns
            .iter()
            .filter(|pat| expr.is_match(pat))
            .count();
        println!("{:?}", possible_patterns);
    }
}
