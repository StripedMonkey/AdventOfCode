use core::num;

use aoc_utils::*;
use regex::Regex;
use rs_2024_03::*;

fn main() {
    let input = rs_2024_03::static_read("input1.txt");
    let re =
        Regex::new(r"(?P<no>don't\(\))|(?P<yes>do\(\))|(mul\((?P<a>\d+),(?P<b>\d+)\))").unwrap();
    let mut enabled = true;
    let answer: isize = re
        .captures_iter(input)
        .map(|m| {
            println!("{:?}", m);
            if let Some(allow_acc) = m.name("yes") {
                println!("Enabling");
                enabled = true;
                return 0;
            }
            if let Some(allow_acc) = m.name("no") {
                println!("Disabling");
                enabled = false;
                return 0;
            }
            if !enabled {
                println!("Skipping {:?}*{:?}", &m[4], &m[5]);
                return 0;
            }
            let a = m.name("a").unwrap().as_str().parse::<isize>().unwrap();
            let b = m.name("b").unwrap().as_str().parse::<isize>().unwrap();
            a * b
        })
        .sum();
    println!("{}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_03::static_read("example2.txt");
        let re =
            Regex::new(r"(?P<no>don't\(\))|(?P<yes>do\(\))|(?P<mult>mul\((\d+),(\d+)\))").unwrap();
        let mut enabled = true;
        let answer: isize = re
            .captures_iter(input)
            .map(|m| {
                println!("{:?}", m);
                if let Some(allow_acc) = m.name("yes") {
                    println!("Enabling");
                    enabled = true;
                    return 0;
                }
                if let Some(allow_acc) = m.name("no") {
                    println!("Disabling");
                    enabled = false;
                    return 0;
                }
                if !enabled {
                    println!("Skipping {:?}*{:?}", &m[4], &m[5]);
                    return 0;
                }
                let a = m[4].parse::<isize>().unwrap();
                let b = m[5].parse::<isize>().unwrap();
                a * b
            })
            .sum();
        println!("{}", answer);
        assert!(answer == 48)
    }
}
