use std::{env, path::PathBuf, str::FromStr};

use itertools::Itertools;

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

pub struct GalaxyImage<const SCALING_FACTOR: usize> {
    img: Vec<Vec<char>>,
    empty_col: Vec<bool>,
    empty_row: Vec<bool>,
}

impl<const SCALING_FACTOR: usize> GalaxyImage<SCALING_FACTOR> {
    pub fn new(img: &str) -> Self {
        let img: Vec<Vec<_>> = img
            .lines()
            .rev()
            .map(|line| line.chars().collect())
            .collect();
        let empty_row = img
            .iter()
            .map(|row| row.iter().all(|c| *c == '.'))
            .collect();
        let empty_col = img.iter().fold(vec![true; img.len()], |acc, row| {
            acc.iter()
                .zip(row.iter())
                .map(|(a, b)| *a && *b == '.')
                .collect()
        });
        Self {
            img,
            empty_col,
            empty_row,
        }
    }

    pub fn product_sum(&self) -> usize {
        let mut distance = 0;
        self.galaxies()
            .cartesian_product(self.galaxies())
            .filter(|(pos1, pos2)| pos1 != pos2 && pos1 < pos2)
            .for_each(|(pos1, pos2)| {
                let pair_distance = self.distance_between(pos1, pos2);
                distance += pair_distance;
            });
        distance
    }

    fn get(&self, x: usize, y: usize) -> Option<(char, usize)> {
        if x >= self.img.len() || y >= self.img.len() {
            return None;
        }
        let distance = (SCALING_FACTOR * (self.empty_col[y] || self.empty_row[x]) as usize).max(1);
        Some((self.img[y][x], distance))
    }

    fn galaxies(&self) -> impl Iterator<Item = (usize, usize)> + '_ + Clone {
        self.img.iter().enumerate().flat_map(|(x, row)| {
            row.iter().enumerate().filter_map(move |(y, c)| {
                if *c != '.' {
                    return Some((x, y));
                }
                None
            })
        })
    }

    fn distance_between(&self, start: (usize, usize), end: (usize, usize)) -> usize {
        let mut distance = 0;
        let mut current = start;
        loop {
            if current == end {
                return distance;
            }
            let (current_x, current_y) = current;
            if current_x < end.0 {
                current.0 += 1;
            } else if current_x > end.0 {
                current.0 -= 1;
            } else if current_y < end.1 {
                current.1 += 1;
            } else if current_y > end.1 {
                current.1 -= 1;
            }
            let Some((_, move_cost)) = self.get(current.0, current.1) else {
                panic!()
            };
            distance += move_cost;
        }
    }
}
