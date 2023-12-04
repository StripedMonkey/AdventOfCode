use std::{env, path::PathBuf, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, u32},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

#[macro_use]
extern crate lazy_static;

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

#[derive(Debug, Clone)]
pub struct ScratchCard {
    card: usize,
    winning_numbers: Vec<u32>, // Sorted!
    your_numbers: Vec<u32>,    // Sorted!
}

impl ScratchCard {
    pub fn points(&self) -> usize {
        self.your_numbers
            .iter()
            .filter(|c| self.winning_numbers.binary_search(c).is_ok())
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
    }
    pub fn matches(&self) -> impl Iterator<Item = &u32> {
        self.your_numbers.iter().filter(|c| {
            self.winning_numbers.binary_search(c).is_ok()
        })
    }
}

pub fn parse_scratchcard(line: &str) -> IResult<&str, ScratchCard> {
    let (line, _) = tag("Card")(line)?;
    let (line, card) = preceded(multispace1, u32)(line)?;
    let (line, _) = terminated(tag(":"), multispace1)(line)?;
    let (line, mut winning_numbers) = separated_list1(multispace1, u32)(line)?;
    let (line, _) = delimited(multispace0, tag("|"), multispace0)(line)?;
    let (line, mut your_numbers) = separated_list1(multispace1, u32)(line)?;
    winning_numbers.sort();
    your_numbers.sort();
    Ok((
        line,
        ScratchCard {
            card: card as usize,
            winning_numbers,
            your_numbers,
        },
    ))
}
