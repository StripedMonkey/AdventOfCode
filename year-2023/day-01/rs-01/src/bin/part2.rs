use std::cmp::{self, Reverse};

use rs_2023_01::*;

const ALPHA_DIGITS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_alpha_digit(line: &str) -> (Option<(u32, usize)>, Option<(u32, usize)>) {
    let mut min: Option<(u32, usize)> = None;
    let mut max: Option<(u32, usize)> = None;
    for (n, n_str) in ALPHA_DIGITS.iter().enumerate() {
        for (idx, _) in line.match_indices(n_str) {
            if let Some(current) = min {
                min = Some(cmp::min_by_key(current, (n as u32, idx), |n| n.1));
            } else {
                min = Some((n as u32, idx));
            }
            if let Some(current) = max {
                max = Some(cmp::max_by_key(
                    current,
                    (n as u32, idx + n_str.len() - 1),
                    |n| n.1,
                ));
            } else {
                max = Some((n as u32, idx + n_str.len() - 1));
            }
        }
    }
    (min, max)
}

fn find_numeric_digit(line: &str) -> (Option<(u32, usize)>, Option<(u32, usize)>) {
    let first_numeric: Option<(u32, usize)> = line
        .chars()
        .position(|c| c.is_numeric())
        .and_then(|idx| Some((line.chars().nth(idx)?.to_digit(10)?, idx)));
    let last_numeric: Option<(u32, usize)> = line
        .chars()
        .rev()
        .position(|c| c.is_numeric())
        .and_then(|idx| {
            Some((
                line.chars().nth(line.len() - 1 - idx)?.to_digit(10)?,
                line.len() - idx,
            ))
        });
    (first_numeric, last_numeric)
}

fn matched_by<O, F, K>(a: Option<(u32, O)>, b: Option<(u32, O)>, f: F) -> Option<u32>
where
    O: Ord + Copy,
    K: Ord,
    F: FnMut(&(u32, O)) -> K,
{
    if let (Some(a), Some(b)) = (a, b) {
        Some(cmp::max_by_key(a, b, f).0)
    } else {
        a.or(b).map(|n| n.0)
    }
}

fn process_line(line: &str) -> Option<u32> {
    let (first_alpha, last_alpha) = find_alpha_digit(line);
    let (first_numeric, last_numeric) = find_numeric_digit(line);
    let first = matched_by(first_alpha, first_numeric, |n| Reverse(n.1))?;
    let last = matched_by(last_alpha, last_numeric, |n| n.1)?;
    let val = (first * 10) + last;
    Some(val)
}

fn main() {
    let input = *INPUT_1;
    let result = input.lines().map_while(process_line).sum::<u32>();
    println!("The answer is {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = rs_2023_01::static_read("example1.txt");
        let result = input.lines().map_while(process_line).sum::<u32>();
        assert_eq!(result, 142);
    }

    #[test]
    fn test_example2() {
        let input = rs_2023_01::static_read("example2.txt");
        let result = input.lines().map_while(process_line).sum::<u32>();
        assert_eq!(result, 281);
    }

    #[test]
    // My original attempt did not account for searching for a repeating digit
    // and only found the first, so extra verification!
    fn test_double_digits() {
        let input = "eighteight";
        let result = input.lines().map_while(process_line).sum::<u32>();
        assert_eq!(result, 88);
    }
}
