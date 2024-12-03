use core::num;

use rs_2024_03::*;
use aoc_utils::*;
use regex::Regex;


fn main() {
    let input = rs_2024_03::static_read("input1.txt");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let answer: isize = re.captures_iter(input).map(|mult| {
        println!("{:?}", mult);
        let a = mult[1].parse::<isize>().unwrap();
        let b = mult[2].parse::<isize>().unwrap();
        a*b
    }).sum();
    println!("{}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_03::static_read("example1.txt");
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let answer: isize = re.captures_iter(input).map(|mult| {
            println!("{:?}", mult);
            let a = mult[1].parse::<isize>().unwrap();
            let b = mult[2].parse::<isize>().unwrap();
            a*b
        }).sum();
        println!("{}", answer);
        assert!(answer == 161)
    }
}
