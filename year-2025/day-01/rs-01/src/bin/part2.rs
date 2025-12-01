use std::ops::{Add, AddAssign, SubAssign};

use aoc_utils::*;
use rs_2025_01::*;

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

struct Dial<const N: usize> {
    position: usize,
}

impl<const N: usize> Dial<N> {
    fn new(starting_position: usize) -> Self {
        Self {
            position: starting_position,
        }
    }
}

impl<const N: usize> AddAssign<usize> for Dial<N> {
    fn add_assign(&mut self, rhs: usize) {
        self.position = (self.position + rhs) % (N + 1);
    }
}

impl<const N: usize> SubAssign<usize> for Dial<N> {
    fn sub_assign(&mut self, rhs: usize) {
        self.position = (self.position + (N + 1) - (rhs % (N + 1))) % (N + 1);
    }
}

impl<const N: usize> Dial<N> {
    fn add(&mut self, n: usize) -> usize {
        let extra = (self.position + n) / (N + 1);
        self.add_assign(n);
        extra
    }

    fn sub(&mut self, n: usize) -> usize {
        let mut extra = n / (N + 1);
        if n % (N + 1) > self.position && self.position != 0 {
            extra += 1;
        }
        self.sub_assign(n);
        if self.position == 0 {
            extra += 1;
        }
        extra
    }
}

fn main() {
    let input = rs_2025_01::static_read("input1.txt");

    let mut dial = Dial::<99>::new(50);
    let rotations = input.lines().map(|line| line.chars()).map(|mut line| {
        let first = line.next().unwrap();
        let direction = match first {
            'L' => Rotation::Left,
            'R' => Rotation::Right,
            _ => panic!("Invalid direction"),
        };
        let value: usize = line.collect::<String>().parse().unwrap();
        (direction, value)
    });

    let mut times_at_zero = 0;
    for (direction, value) in rotations {
        let starting_position = dial.position;
        let extra = match direction {
            Rotation::Left => dial.sub(value),
            Rotation::Right => dial.add(value),
        };
        if extra > 1 {
            println!(
                "{direction:?} {value} {starting_position} -> {}: passing zero {} times",
                dial.position, extra
            );
        }
        times_at_zero += extra;
    }
    println!("{}", times_at_zero);
    assert_eq!(times_at_zero, 5847);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2025_01::static_read("example1.txt");
        let mut dial = Dial::<99>::new(50);
        let rotations = input.lines().map(|line| line.chars()).map(|mut line| {
            let first = line.next().unwrap();
            let direction = match first {
                'L' => Rotation::Left,
                'R' => Rotation::Right,
                _ => panic!("Invalid direction"),
            };
            let value: usize = line.collect::<String>().parse().unwrap();
            (direction, value)
        });
        let rotations: Vec<_> = rotations.collect();
        eprintln!("{rotations:?}");
        let mut times_at_zero = 0;
        for (direction, value) in rotations {
            let starting_position = dial.position;
            let extra = match direction {
                Rotation::Left => dial.sub(value),
                Rotation::Right => dial.add(value),
            };
            println!(
                "{direction:?} {value} {starting_position} -> {}: passing zero {} times",
                dial.position, extra
            );
            times_at_zero += extra;
        }
        println!("{}", times_at_zero);

        assert!(times_at_zero == 6)
    }

    #[test]
    fn edge_maybe() {
        let input = "L400
R400
L250
R300
R50
R50
L30
L30";
        let mut dial = Dial::<99>::new(50);
        let rotations = input.lines().map(|line| line.chars()).map(|mut line| {
            let first = line.next().unwrap();
            let direction = match first {
                'L' => Rotation::Left,
                'R' => Rotation::Right,
                _ => panic!("Invalid direction"),
            };
            let value: usize = line.collect::<String>().parse().unwrap();
            (direction, value)
        });

        let mut times_at_zero = 0;
        for (direction, value) in rotations {
            let starting_position = dial.position;
            let extra = match direction {
                Rotation::Left => dial.sub(value),
                Rotation::Right => dial.add(value),
            };
            println!(
                "{direction:?} {value} {starting_position} -> {}: passing zero {} times",
                dial.position, extra
            );
            times_at_zero += extra;
        }
        println!("{}", times_at_zero);
        assert_eq!(times_at_zero, 15)
    }

    #[test]
    fn edge_case() {
        let mut dial = Dial::<99>::new(50);
        let extra = dial.sub(150);
        assert_eq!(extra, 2);
    }

    #[test]
    fn edge_case1() {
        let mut dial = Dial::<99>::new(50);
        let extra = dial.sub(400);
        eprintln!("Position after sub: {}", dial.position);
        assert_eq!(extra, 4);
    }

    #[test]
    fn edge_case2() {
        let mut dial = Dial::<99>::new(50);
        let extra = dial.sub(68);
        eprintln!("Position after sub: {}", dial.position);
        assert_eq!(extra, 1);
    }

    #[test]
    fn edge_case3() {
        let mut dial = Dial::<99>::new(50);
        let extra = dial.sub(268);
        eprintln!("Position after sub: {}", dial.position);
        assert_eq!(extra, 3);
    }
}
