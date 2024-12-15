use std::collections::HashMap;

use aoc_utils::*;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res, opt},
    number,
    sequence::{separated_pair, tuple},
    IResult,
};
use rs_2024_14::*;

fn main() {
    let input = rs_2024_14::static_read("input1.txt");
    let mut robots = parse(&input);
    let room_size = (101, 103);
    // I got the suggestion of entropy being the answer
    // Assuming a random distribution of robots, the chance of a robot being in a particular square is #bots / area
    let answer = (1..10_000)
        .map(|i| {
            for robot in robots.iter_mut() {
                robot.step(1, room_size);
            }
            (i, map_entropy(&robots, room_size))
        })
        .fold((0, f64::NAN), |acc, (i, entropy)| {
            if f64::max(acc.1, entropy) == entropy {
                return (i, entropy);
            }
            acc
        });
    println!("{:?}", answer);
}

fn map_entropy(robots: &[Robot], room_size: (usize, usize)) -> f64 {
    let area = room_size.0 * room_size.1;
    let num_robots = robots.len();
    let expected_probability = num_robots as f64 / area as f64;
    let entropy: f64 = robots
        .iter()
        .map(|r| r.position)
        .counts()
        .iter()
        .map(|(_, c)| {
            let p = *c as f64 / num_robots as f64;
            p * (p / expected_probability).log2()
        })
        .sum();
    -entropy
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_14::static_read("example1.txt");
        let mut robots = parse(&input);
        let room_size = (11, 7);
        print_room(&robots, room_size);
        for robot in robots.iter_mut() {
            robot.step(100, room_size);
        }
        let mut counts = [[0; 2]; 2];
        for robot in robots.iter() {
            if (robot.position.0 as usize) == room_size.0 / 2 {
                continue;
            }
            if (robot.position.1 as usize) == room_size.1 / 2 {
                continue;
            }
            if (robot.position.0 as usize) < room_size.0 / 2 {
                if (robot.position.1 as usize) < room_size.1 / 2 {
                    counts[0][0] += 1;
                } else {
                    counts[0][1] += 1;
                }
            } else {
                if (robot.position.1 as usize) < room_size.1 / 2 {
                    counts[1][0] += 1;
                } else {
                    counts[1][1] += 1;
                }
            }
        }
        println!("{:?}", counts);
        println!("{:?}", counts.iter().flatten().product::<usize>())
    }

    #[test]
    fn second_test() {
        let mut robots = vec![Robot {
            position: (2, 4),
            velocity: (2, -3),
        }];
        let room_size = (11, 7);
        for _ in 0..4 {
            for robot in robots.iter_mut() {
                robot.step(1, room_size);
            }
        }
    }

    #[test]
    fn third_test() {
        let input = rs_2024_14::static_read("input1.txt");
        let mut robots = parse(&input);
        let room_size = (101, 103);
        for robot in robots.iter_mut() {
            robot.step(7790, room_size);
        }
        print_room(&robots, room_size);
        let entropy = map_entropy(&robots, room_size);
        println!("{}", entropy);
    }
}

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

fn print_room(robots: &[Robot], room_size: (usize, usize)) {
    for y in 0..room_size.1 {
        for x in 0..room_size.0 {
            let num_robots = robots
                .iter()
                .filter(|r| r.position == (x as isize, y as isize))
                .count();
            if num_robots > 0 {
                print!("{}", num_robots);
                continue;
            }
            print!(".");
        }
        println!();
    }
}

impl Robot {
    fn step(&mut self, steps: usize, room_size: (usize, usize)) {
        self.position.0 += self.velocity.0 * steps as isize;
        self.position.1 += self.velocity.1 * steps as isize;
        self.position.0 %= room_size.0 as isize;
        if self.position.0 < 0 {
            self.position.0 += room_size.0 as isize;
        }
        self.position.1 %= room_size.1 as isize;
        if self.position.1 < 0 {
            self.position.1 += room_size.1 as isize;
        }
    }
}
fn number(input: &str) -> IResult<&str, isize> {
    let (input, sign) = opt(tag("-"))(input)?;
    let (input, number) = map_res(digit1, |n| isize::from_str_radix(n, 10))(input)?;
    Ok((input, sign.map_or(Ok(number), |_| Ok(-number))?))
}
// p=0,4 v=3,-3
fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, _) = tag("p=")(input)?;
    let (input, position) = separated_pair(number, tag(","), number)(input)?;
    let (input, _) = tag(" v=")(input)?;
    let (input, velocity) = separated_pair(number, tag(","), number)(input)?;
    Ok((
        input,
        Robot {
            position: (position.0, position.1),
            velocity,
        },
    ))
}

fn parse(input: &str) -> Vec<Robot> {
    let robots: Result<_, _> = input
        .lines()
        .map(|line| all_consuming(robot)(line).map(|(_, r)| r))
        .collect();
    robots.unwrap()
}
