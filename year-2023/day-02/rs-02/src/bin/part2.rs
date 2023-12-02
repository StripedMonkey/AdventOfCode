use itertools::Itertools;
use rs_02::*;

use nom::{
    self,
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub struct Game<'a> {
    n: usize,
    rounds: Vec<Vec<(u64, &'a str)>>,
}

impl Game<'_> {
    pub fn lower_bound(&self) -> (u64, u64, u64) {
        self.rounds
            .iter()
            .map(round_rgb)
            .fold((0, 0, 0), |acc, new| {
                let (x, y, z) = acc;
                let (i, j, k) = new;
                (x.max(i), y.max(j), z.max(k))
            })
    }

    pub fn lower_power(&self) -> u64 {
        let (red, green, blue) = self.lower_bound();
        red * green * blue
    }
}

fn round_rgb(round: &Vec<(u64, &str)>) -> (u64, u64, u64) {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for (n, color) in round {
        match *color {
            "red" => red += n,
            "green" => green += n,
            "blue" => blue += n,
            _ => panic!("Unknown color"),
        }
    }
    (red, green, blue)
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn process_game<'a>(line: &'a str) -> IResult<&str, Game<'a>> {
    let (line, _) = tag("Game ")(line)?;
    let (line, n) = u64(line)?;
    let (line, _) = tag(": ")(line)?;
    let (line, rounds) = {
        let (line, rounds) = separated_list1(
            tag("; "),
            separated_list1(tag(", "), separated_pair(u64, tag(" "), alpha1)),
        )(line)?;
        (line, rounds)
    };

    let game = Game {
        n: n as usize,
        rounds,
    };
    return Ok((line, game));
}

fn main() {
    // Find only games which can contain 12 red, 13 green, and 14 blue
    let _file = *INPUT_1;
    let num: u64 = _file
        .lines()
        .map(|line| process_game(line))
        .map(|game| game.unwrap().1.lower_power())
        .sum();
    println!("{}", num);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let _file: &str = rs_02::static_read("example2.txt");
        let num: u64 = _file
            .lines()
            .map(|line| process_game(line).unwrap().1)
            .map(|game| game.lower_power())
            .sum();
        assert_eq!(num, 2286)
    }
}
