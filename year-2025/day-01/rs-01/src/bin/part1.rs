
use std::ops::{Add, AddAssign, SubAssign};

use aoc_utils::*;
use rs_2025_01::*;

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
        self.position = (self.position + rhs) % (N+1);
    }
}

impl<const N: usize> SubAssign<usize> for Dial<N> {
    fn sub_assign(&mut self, rhs: usize) {
        self.position = (self.position + (N+1) - (rhs % (N +1))) % (N + 1);
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
            match direction {
                Rotation::Left => dial -= value,
                Rotation::Right => dial += value,
            }
            if dial.position == 0 {
                times_at_zero += 1;
            }
        }
        println!("{}", times_at_zero);

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

        let mut times_at_zero = 0;
        for (direction, value) in rotations {
            println!("Dial at position: {}", dial.position);
            match direction {
                Rotation::Left => dial -= value,
                Rotation::Right => dial += value,
            }
            if dial.position == 0 {
                times_at_zero += 1;
            }
        }
        println!("{}", times_at_zero);
        assert!(times_at_zero == 3);

    }
}
