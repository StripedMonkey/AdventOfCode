#[macro_use]
extern crate lazy_static;

use std::{env, path::PathBuf, str::FromStr};

use nom::{
    self,
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub type Rgb = (u64, u64, u64);
type NomError<'a, T> = nom::Err<nom::error::Error<T>>;

// The input texts are static, should it be? Probably not, but it was an excuse to do it this way.
lazy_static! {
    pub static ref INPUT_1: &'static str = static_read("input1.txt");
    pub static ref INPUT_2: &'static str = static_read("input2.txt");
}

// Read a file path relative to the parent of the current day's project directory.
// Each day contains the inputs and examples for that day in the the parent, so I can
// (in theory/later) share the inputs between multiple languages. Pretend like I'll actually do that.
pub fn static_read(file_path: &str) -> &'static str {
    let mut cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).expect("Infallible action failed!");
    cwd.pop();
    let file_path = cwd.join(file_path);
    let file = std::fs::read_to_string(file_path).expect("Failed to open file!");
    Box::leak(file.into_boxed_str())
}

#[derive(Debug)]
pub struct Game<'a> {
    pub n: usize,
    rounds: Vec<Vec<(u64, &'a str)>>,
}

fn parse_game<'a>(line: &'a str) -> IResult<&str, Game<'a>> {
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

// Find the number of red, green, and blue widgets in a round.
pub fn round_rgb(round: &Vec<(u64, &str)>) -> (u64, u64, u64) {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for (n, color) in round {
        match *color {
            "red" => red += n,
            "green" => green += n,
            "blue" => blue += n,
            _ => {}
        }
    }
    (red, green, blue)
}

impl<'i> Game<'i> {
    /// Find the minimum number of red, green, and blue widgets required for a round to have occurred
    pub fn lower_rgb_bound(&self) -> (u64, u64, u64) {
        self.rounds
            .iter()
            .map(round_rgb)
            .fold((0, 0, 0), |acc, new| {
                let (x, y, z) = acc;
                let (i, j, k) = new;
                (x.max(i), y.max(j), z.max(k))
            })
    }

    /// Find the lower bound's "power"
    pub fn lower_rgb_power(&self) -> u64 {
        let (red, green, blue) = self.lower_rgb_bound();
        red * green * blue
    }

    pub fn from_str(line: &'i str) -> Result<Self, NomError<&'i str>> {
        let (_, game) = parse_game(line)?;
        Ok(game)
    }

    pub fn iter_rounds(&self) -> impl Iterator<Item = &Vec<(u64, &'i str)>> {
        self.rounds.iter()
    }

    pub fn possible_game(&self, rgb: Rgb) -> bool {
        let (max_red, max_green, max_blue) = rgb;
        self.iter_rounds().all(|round| {
            let (red, green, blue) = round_rgb(round);
            (red <= max_red) && (green <= max_green) && (blue <= max_blue)
        })
    }
}
