use core::num;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
};

use aoc_utils::*;
use rs_2024_19::*;

fn main() {
    let input = rs_2024_19::static_read("input1.txt");
    let (available_towels, desired_patterns) = parse(&input);
    let mut memoized: HashMap<&str, usize> = HashMap::new();
    let mut queue: VecDeque<_> = desired_patterns.iter().map(|s| s.as_str()).collect();
    'counting_patterns: while let Some(pattern) = queue.pop_back() {
        if memoized.contains_key(pattern) {
            // A memoized pattern can just be accumulated and done
            continue;
        }
        queue.push_back(pattern); // We don't know how to handle this pattern yet
        let mut counted_patterns = 0;
        for towel in &available_towels {
            if pattern.starts_with(towel) {
                let ending_pattern = &pattern[towel.len()..];
                if memoized.contains_key(ending_pattern) {
                    counted_patterns += memoized[ending_pattern];
                } else if ending_pattern.is_empty() {
                    counted_patterns += 1;
                } else {
                    queue.push_back(ending_pattern);
                    continue 'counting_patterns;
                }
            }
        }
        memoized.insert(pattern, counted_patterns);
    }
    let total_combinations: usize = memoized
        .iter()
        .filter(|(k, _)| desired_patterns.contains(&k.to_string()))
        .map(|(_, v)| v)
        .sum();
    // println!("{:?}", memoized);
    println!("{:?}", total_combinations);
    assert_eq!(total_combinations, 622121814629343);
}

#[cfg(test)]
mod test {
    use std::collections::{HashSet, VecDeque};

    use itertools::Itertools;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_19::static_read("example1.txt");
        let (available_towels, desired_patterns) = parse(&input);
        let mut queue: VecDeque<_> = desired_patterns.iter().map(|s| s.as_str()).collect();
        let mut num_combinations = 0;
        while let Some(pattern) = queue.pop_back() {
            available_towels.iter().for_each(|towel| {
                if pattern.starts_with(towel) {
                    let new_pattern = &pattern[towel.len()..];
                    if new_pattern.is_empty() {
                        num_combinations += 1;
                    } else {
                        println!("{:?}", new_pattern);
                        queue.push_back(new_pattern);
                    }
                }
            });
        }
        println!("{:?}", num_combinations);
    }

    #[test]
    fn second_test() {
        let input = rs_2024_19::static_read("example1.txt");
        let (available_towels, desired_patterns) = parse(&input);
    }
}
