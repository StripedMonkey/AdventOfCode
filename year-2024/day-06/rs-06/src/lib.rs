#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use std::{cmp::Ordering, env, path::PathBuf, str::FromStr};
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

pub enum Step {
    Rotate(char),
    Move((usize, usize)),
    End,
}

pub fn step(map: &Vec<Vec<char>>, current_position: (usize, usize)) -> Step {
    let directions: Vec<(char, (isize, isize))> =
        vec![('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))];
    let c = map[current_position.0][current_position.1];
    let Some((idx, (_, move_dir))) = directions.iter().find_position(|(d, _)| *d == c) else {
        panic!();
    };
    let Some(new_pos) = map
        .get(current_position.0.wrapping_add_signed(move_dir.0))
        .and_then(|v| v.get(current_position.1.wrapping_add_signed(move_dir.1)))
    else {
        return Step::End;
    };
    match new_pos {
        '#' => {
            let Some((new_dir, _)) = directions.get((idx + 1) % directions.len()) else {
                panic!();
            };
            Step::Rotate(*new_dir)
        }
        _ => Step::Move((current_position.0.wrapping_add_signed(move_dir.0), current_position.1.wrapping_add_signed(move_dir.1))),
    }
}

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|group| group.chars().collect()).collect()
}
