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
struct Game<'a> {
    n: usize,
    rounds: Vec<Vec<(u64, &'a str)>>,
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn process_line<'a>(line: &'a str) -> IResult<&str, Game<'a>> {
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
    println!("{game:?}");
    return Ok((line, game));
}

fn main() {
    // Find only games which can contain 12 red, 13 green, and 14 blue
    let _file = *INPUT_1;
    let num: usize = _file
        .lines()
        .map(|line| process_line(line).unwrap().1)
        .filter(|game| {
            game.rounds.iter().all(|round| {
                println!("Round: {round:?}");
                let (mut red, mut green, mut blue) = (0, 0, 0);
                for (n, color) in round {
                    match *color {
                        "red" => red += n,
                        "green" => green += n,
                        "blue" => blue += n,
                        _ => panic!("Unknown color"),
                    }
                }
                println!("Total: {} {} {}", red, green, blue);
                let x = (red <= 12) && (green <= 13) && (blue <= 14);
                println!("Possible? {x:?}");
                x
            })
        })
        .map(|game| game.n)
        .sum();
    println!("{}", num);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        
    }
}
