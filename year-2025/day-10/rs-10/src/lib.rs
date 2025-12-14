#[macro_use]
extern crate lazy_static;

use std::{env, path::PathBuf, str::FromStr};

// The input texts are static, should it be? Probably not, but it was an excuse to do it this way.
lazy_static! {
    pub static ref INPUT_1: &'static str = static_read("input1.txt");
}

// Read a file path relative to the parent of the current day's project directory.
// Each day contains the inputs and examples for that day in the the parent, so I can
// (in theory/later) share the inputs between multiple languages. Pretend like I'll actually do that.
pub fn static_read(file_path: &str) -> &'static str {
    let mut cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).expect("Infallible action failed!");
    cwd.pop();
    let file_path = cwd.join(file_path);
    println!("{:?}", file_path);
    let file = std::fs::read_to_string(file_path).expect("Failed to open file!");
    Box::leak(file.into_boxed_str())
}

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::space1,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<u16>>> {
    separated_list1(
        space1,
        delimited(
            tag("("),
            separated_list1(tag(","), nom::character::complete::u16),
            tag(")"),
        ),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<bool>, Vec<Vec<u16>>, Vec<u32>)> {
    let (rest, indicator_lights) = delimited(
        tag("["),
        map(take_till(|c| c == ']'), |s: &str| {
            s.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect::<Vec<bool>>()
        }),
        tag("]"),
    )(input)?;
    let (rest, _) = space1(rest)?;
    let (rest, buttons) = parse_buttons(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, joltage_requirements) = delimited(
        tag("{"),
        separated_list1(tag(","), nom::character::complete::u32),
        tag("}"),
    )(rest)?;
    Ok((rest, (indicator_lights, buttons, joltage_requirements)))
}

fn _parse(input: &str) -> IResult<&str, Vec<Machine>> {
    let mut machines = Vec::new();
    for line in input.lines() {
        let (_rest, (indicator_lights, buttons, joltage_requirements)) =
            all_consuming(parse_line)(line)?;
        machines.push(Machine {
            indicator_lights,
            buttons,
            joltage_requirements,
        });
    }
    Ok(("", machines))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    all_consuming(_parse)(input)
}


pub struct Machine {
    indicator_lights: Vec<bool>,
    buttons: Vec<Vec<u16>>,
    joltage_requirements: Vec<u32>,
}

impl Machine {


    pub fn get_button_masks(&self) -> Vec<u16> {
        self.buttons
            .iter()
            .map(|button| {
                button
                    .iter()
                    .fold(0u16, |acc, &light_index| acc | (1 << light_index))
            })
            .collect()
    }

    pub fn get_target_joltage(&self) -> Vec<u32> {
        self.joltage_requirements.clone()
    }

    pub fn get_target_indicator(&self) -> Vec<bool> {
        self.indicator_lights.clone()
    }

    pub fn get_buttons_joltage(&self) -> &Vec<Vec<u16>> {
        &self.buttons
    }
}

