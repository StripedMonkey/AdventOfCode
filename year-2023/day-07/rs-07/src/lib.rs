use std::{env, path::PathBuf, str::FromStr};

use nom::{character::complete::alphanumeric1, IResult};

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

pub fn parse_hand(line: &str) -> IResult<&str, CardHand> {
    let (line, hand) = alphanumeric1(line)?;
    let (line, _) = nom::character::complete::space1(line)?;
    let (line, bid) =
        nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse::<u64>())(
            line,
        )?;
    Ok((line, CardHand { hand: hand, bid }))
}

#[derive(Debug, Eq, PartialEq)]
pub struct CardHand<'a> {
    hand: &'a str,
    bid: u64,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOFAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

trait HandValue {
    fn hand_type(&self) -> HandType;
    fn card_value() -> usize;
}
