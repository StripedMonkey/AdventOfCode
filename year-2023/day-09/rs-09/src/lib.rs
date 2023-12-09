use std::{env, path::PathBuf, str::FromStr};

use nom::{
    character::complete::{i64, space1},
    multi::separated_list1,
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

pub struct History {
    sequence: Vec<i64>,
}

impl History {
    pub fn next(&self) -> i64 {
        let mut differences = Vec::new();

        differences.push(self.sequence.clone());
        loop {
            let last = differences.last().unwrap();
            let next = Self::difference(last);
            if next.iter().all(|v| *v == 0) {
                differences.push(next);
                break;
            }
            differences.push(next);
        }
        // differences.reverse();
        for i in (0..differences.len()).rev() {
            let last_elem = *differences[i].last().unwrap();
            let inc = if differences.len() - 1 == i {
                0
            } else {
                *differences[i + 1].last().unwrap()
            };
            differences[i].push(last_elem + inc);
        }
        *differences.first().unwrap().last().unwrap()
    }

    pub fn previous(&self) -> i64 {
        let mut seq = self.sequence.clone();
        seq.reverse();
        let hist = History { sequence: seq };
        hist.next()
    }

    fn difference(values: &Vec<i64>) -> Vec<i64> {
        values
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect()
    }
}

pub fn parse_history(line: &str) -> IResult<&str, History> {
    let (line, sequence) = separated_list1(space1, i64)(line)?;
    Ok((line, History { sequence }))
}
