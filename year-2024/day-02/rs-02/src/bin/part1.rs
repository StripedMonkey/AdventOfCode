use core::num;

use aoc_utils::*;
use itertools::Itertools;
use rs_2024_02::*;

fn main() {
    let input = rs_2024_02::static_read("input1.txt");
    let data = parse(input);
    let ans = data.iter().filter(|report| {
        if report[0]==report[1] {
            return false;
        }
        if report[0] > report[1] {
            for i in 1..report.len() {
                if report[i-1] <= report[i] {
                    return false;
                }
                match report[i-1].abs_diff(report[i]) {
                    1..=3 => continue,
                    _ => return false,
                }
            }
        }
        if report[0] < report[1] {
            for i in 1..report.len() {
                if report[i-1] >= report[i] {
                    return false;
                }
                match report[i-1].abs_diff(report[i]) {
                    1..=3 => continue,
                    _ => return false,
                }
            }
        }
        return true;
    }).count();
    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_02::static_read("example1.txt");
        let data = parse(input);
        let ans = data.iter().filter(|report| {
            if report[0]==report[1] {
                return false;
            }
            if report[0] > report[1] {
                for i in 1..report.len() {
                    if report[i-1] <= report[i] {
                        return false;
                    }
                    match report[i-1].abs_diff(report[i]) {
                        1..=3 => continue,
                        _ => return false,
                    }
                }
            }
            if report[0] < report[1] {
                for i in 1..report.len() {
                    if report[i-1] >= report[i] {
                        return false;
                    }
                    match report[i-1].abs_diff(report[i]) {
                        1..=3 => continue,
                        _ => return false,
                    }
                }
            }
            return true;
        }).count();
        println!("{}", ans);
        assert!(ans == 2)
    }
}
