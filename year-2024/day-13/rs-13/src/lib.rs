use std::{path::PathBuf, str::FromStr};

use itertools::Itertools as _;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{self, preceded},
    IResult,
};
use num::ToPrimitive as _;
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

#[derive(Debug)]
pub struct ArcadeMachine {
    pub button_a: (usize, usize),
    pub button_b: (usize, usize),
    pub prize: (usize, usize),
}

pub fn intersection(
    slope_a: (usize, usize),
    slope_b: (usize, usize),
    target: (usize, usize),
) -> Option<(usize, usize)> {
    let (ax, ay) = slope_a;
    let (ax, ay) = (ax as i128, ay as i128);
    let (bx, by) = slope_b;
    let (bx, by) = (bx as i128, by as i128);
    let (tx, ty) = target;
    let (tx, ty) = (tx as i128, ty as i128);
    let B = (ty * ax - ay * tx) / (by * ax - ay * bx);
    let A = (tx - bx * B) / ax;
    if B < 0 || A < 0 {
        return None;
    }
    if A * ax + B * bx == tx && A * ay + B * by == ty {
        return Some((A as usize, B as usize));
    }
    None
}

/*
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
 */
fn arcade_machine(input: &str) -> IResult<&str, ArcadeMachine> {
    let (input, ax) = preceded(
        tag("Button A: "),
        preceded(tag("X+"), map_res(digit1, |d| usize::from_str_radix(d, 10))),
    )(input)?;
    let (input, ay) = preceded(
        tag(", Y+"),
        map_res(digit1, |d| usize::from_str_radix(d, 10)),
    )(input)?;
    let (input, _) = multispace1(input)?;

    let (input, bx) = preceded(
        tag("Button B: "),
        preceded(tag("X+"), map_res(digit1, |d| usize::from_str_radix(d, 10))),
    )(input)?;
    let (input, by) = preceded(
        tag(", Y+"),
        map_res(digit1, |d| usize::from_str_radix(d, 10)),
    )(input)?;
    let (input, _) = multispace1(input)?;

    let (input, px) = preceded(
        tag("Prize: X="),
        map_res(digit1, |d| usize::from_str_radix(d, 10)),
    )(input)?;
    let (input, py) = preceded(
        tag(", Y="),
        map_res(digit1, |d| usize::from_str_radix(d, 10)),
    )(input)?;

    Ok((
        input,
        ArcadeMachine {
            prize: (px, py),
            button_a: (ax, ay),
            button_b: (bx, by),
        },
    ))
}

pub fn parse(input: &str) -> Vec<ArcadeMachine> {
    separated_list1(multispace1, arcade_machine)(input)
        .unwrap()
        .1
}
