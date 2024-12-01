use core::num;

use rs_2024_01::*;
const MAX_RGB: Rgb = (12, 13, 14);

fn main() {
    let input = rs_2024_01::static_read("input1.txt");
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    let ans: usize = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_01::static_read("example1.txt");
        let (mut left, mut right) = parse(input);
        left.sort();
        right.sort();
        let ans: usize = left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum();
        println!("{}", ans);
        assert!(ans == 11)
    }
}
