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
}

// Read a file path relative to the parent of the current day's project directory.
// Each day contains the inputs and examples for that day in the the parent, so I can
// (in theory/later) share the inputs between multiple languages. Pretend like I'll actually do that.
pub fn static_read(file_path: &str) -> &'static str {
    let mut cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).expect("Infallible action failed!");
    cwd.pop();
    let file_path = cwd.join(file_path);
    println!("{:?}",file_path);
    let file = std::fs::read_to_string(file_path).expect("Failed to open file!");
    Box::leak(file.into_boxed_str())
}


pub fn parse(input: &str) -> (Vec<usize>,Vec<usize>) {
    let data = input.lines().map(|line| {
        let line: Vec<usize> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
        (line[0],line[1])
    }).fold((Vec::new(),Vec::new()),|mut a,b| {
        a.0.push(b.0);
        a.1.push(b.1);
        a
    });

    data
}