use core::num;

use aoc_utils::*;
use itertools::Itertools;
use rs_2024_02::*;

fn main() {
    let input = rs_2024_02::static_read("input1.txt");
    let data = parse(input);
    let ans = data
        .iter()
        .filter(|report| {
            valid_report(&report, true).is_ok() || valid_report(&report, false).is_ok()
        })
        .count();
    println!("{}", ans);
    assert!(ans == 287);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_02::static_read("example1.txt");
        let data = parse(input);
        let ans = data
            .iter()
            .filter(|report| {
                valid_report(&report, true).is_ok() || valid_report(&report, false).is_ok()
            })
            .count();
        println!("{}", ans);
        assert!(ans == 2)
    }
}
