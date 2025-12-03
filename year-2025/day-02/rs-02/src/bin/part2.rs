use aoc_utils::*;
use rs_2025_02::*;

fn main() {
    let input = rs_2025_02::static_read("input1.txt");
    let input = input.trim();
    let mut invalid_ids = vec![];
    for range in input.split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();
        for id_num in start..=end {
            let id = format!("{}", id_num);
            if is_invalid(&id) {
                invalid_ids.push(id_num);
            }
        }
    }
    let result = invalid_ids.iter().sum::<usize>();
    println!("{}", result);
}

fn is_invalid(id: &str) -> bool {
    let len = id.len();
    let max_pattern_size = len / 2;
    for pattern_size in 1..=max_pattern_size {
        if len % pattern_size != 0 {
            continue;
        }
        let pattern_to_match = &id[..pattern_size];
        if (pattern_size..len)
            .step_by(pattern_size)
            .all(|j| &id[j..j + pattern_size] == pattern_to_match)
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2025_02::static_read("example1.txt");
        let input = input.trim();
        let mut invalid_ids = vec![];
        for range in input.split(",") {
            let (start, end) = range.split_once("-").unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            for id_num in start..=end {
                let id = format!("{}", id_num);
                if is_invalid(&id) {
                    println!("Invalid ID: {}", id);
                    invalid_ids.push(id_num);
                }
            }
        }
        let result = invalid_ids.iter().sum::<usize>();
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn is_invalid_id() {
        assert!(!is_invalid("1000"));
        assert!(is_invalid("111"));
        assert!(!is_invalid("2121212118"));
        assert!(is_invalid("1010"));
        assert!(is_invalid("1188511885"));
        assert!(is_invalid("2121212121"));
    }
}
