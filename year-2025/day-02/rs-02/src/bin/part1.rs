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
    if len % 2 != 0 {
        return false;
    }
    let split = len / 2;
    id[..split] == id[split..]
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
                    invalid_ids.push(id_num);
                }
            }
        }
        let result = invalid_ids.iter().sum::<usize>();
        assert_eq!(result, 1227775554);
    }
}
