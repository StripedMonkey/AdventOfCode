use std::{fmt::Display, mem, path::PathBuf, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;

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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum RockType {
    Rounded,
    Cube,
    Empty,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ReflectorMap {
    grid: Vec<Vec<RockType>>,
}

impl ReflectorMap {
    pub fn height(&self) -> usize {
        self.grid.len()
    }
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn tilt_north(&self) -> ReflectorMap {
        self.tilt_rows((0..self.height()).tuple_windows())
    }

    pub fn tilt_cycles(&self, num_cycles: usize) -> ReflectorMap {
        let mut sequence: Vec<ReflectorMap> = Vec::new();
        let mut current_cycles = 0;
        sequence.push(self.clone());
        let last = loop {
            if current_cycles > num_cycles {
                return sequence[num_cycles].clone();
            }
            let tilted = sequence.last().unwrap();
            let north = tilted.tilt_rows((0..tilted.height()).tuple_windows());
            let west = north.tilt_cols((0..north.width()).tuple_windows());
            let south = west.tilt_rows((0..west.height()).rev().tuple_windows());
            let east = south.tilt_cols((0..south.width()).rev().tuple_windows());

            current_cycles += 1;
            if sequence.contains(&east) {
                println!("Found cycle after {} rotations", current_cycles);
                break east;
            }
            sequence.push(east);
        };
        let Some((start_idx, _)) = sequence.iter().find_position(|x| **x == last) else {
            panic!()
        };
        let cycle_len = sequence.len() - start_idx;

        let end_idx = start_idx + ((num_cycles - start_idx) % cycle_len);
        println!(
            "Num loops: {} Cycle length: {}, end index: {}",
            sequence.len(),
            cycle_len,
            end_idx
        );
        println!("{}", sequence[end_idx]);
        sequence[end_idx].clone()
    }

    fn tilt_rows(&self, dir: impl Iterator<Item = (usize, usize)> + Clone) -> ReflectorMap {
        let mut tilted_map = self.clone();
        let mut swapped = true;
        while swapped {
            swapped = false;
            for (a, b) in dir.clone() {
                swapped = tilted_map.tilt_row(a, b) || swapped;
            }
        }
        tilted_map
    }

    fn tilt_cols(&self, dir: impl Iterator<Item = (usize, usize)> + Clone) -> ReflectorMap {
        let mut tilted_map = self.clone();
        let mut swapped = true;
        while swapped {
            swapped = false;
            for (a, b) in dir.clone() {
                swapped = tilted_map.tilt_col(a, b) || swapped;
            }
        }
        tilted_map
    }

    fn tilt_row(&mut self, idx_a: usize, idx_b: usize) -> bool {
        if idx_a > idx_b {
            let (lower, upper) = self.grid.split_at_mut(idx_a);
            let a = upper[0].iter_mut();
            let b = lower[idx_b].iter_mut();
            tilt(a, b, try_swap)
        } else {
            let (lower, upper) = self.grid.split_at_mut(idx_b);
            let a = lower[idx_a].iter_mut();
            let b = upper[0].iter_mut();
            tilt(a, b, try_swap)
        }
    }

    fn tilt_col(&mut self, idx_a: usize, idx_b: usize) -> bool {
        self.grid
            .iter_mut()
            .map(|row| {
                let (a, b) = if idx_a > idx_b {
                    let (lower, upper) = row.split_at_mut(idx_a);
                    (&mut upper[0], &mut lower[idx_b])
                } else {
                    let (lower, upper) = row.split_at_mut(idx_b);
                    (&mut lower[idx_a], &mut upper[0])
                };
                try_swap(a, b)
            })
            .fold(false, |acc, x| acc || x)
    }

    pub fn load(&self) -> usize {
        let x = (1..=self.grid.len()).rev();
        self.grid
            .iter()
            .map(|row| row.iter().filter(|c| **c == RockType::Rounded).count())
            .zip(x)
            .map(|(x, y)| x * y)
            .sum()
    }
}

fn tilt<'a, T: 'a, It1, It2, F>(it1: It1, it2: It2, try_swap: F) -> bool
where
    It1: Iterator<Item = &'a mut T>,
    It2: Iterator<Item = &'a mut T>,
    F: Fn(&mut T, &mut T) -> bool,
{
    let mut swapped = false;
    it1.zip(it2).for_each(|(a, b)| {
        swapped = try_swap(a, b) || swapped;
    });
    swapped
}

fn try_swap(a: &mut RockType, b: &mut RockType) -> bool {
    if *a == RockType::Empty && *b == RockType::Rounded {
        mem::swap(a, b);
        return true;
    }
    return false;
}

pub fn parse_file(file: &str) -> ReflectorMap {
    let mut map = Vec::new();
    for line in file.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                'O' => row.push(RockType::Rounded),
                '#' => row.push(RockType::Cube),
                '.' => row.push(RockType::Empty),
                _ => panic!("Unknown character"),
            }
        }
        map.push(row);
    }
    ReflectorMap { grid: map }
}

impl Display for RockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RockType::Rounded => write!(f, "o"),
            RockType::Cube => write!(f, "#"),
            RockType::Empty => write!(f, "."),
        }
    }
}
impl Display for ReflectorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
