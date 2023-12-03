use std::{env, fmt::Display, path::PathBuf, str::FromStr};

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

#[derive(Debug)]
pub struct Schematic<'a> {
    board: &'a str,
}

impl Display for Schematic<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}

struct AdjacentNumberIterator<'a> {
    schematic: &'a Schematic<'a>,
    centerpoint: (usize, usize),
    to_check: Vec<(isize, isize)>,
}

impl<'a> Iterator for AdjacentNumberIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (signed_dx, signed_dy) = self.to_check.pop()?;
            let (Some(check_x), Some(check_y)) = (
                self.centerpoint.0.checked_add_signed(signed_dx),
                self.centerpoint.1.checked_add_signed(signed_dy),
            ) else {
                continue;
            };

            if let Some(n) = self.schematic.get_loc(check_x, check_y) {
                if !n.is_numeric() {
                    continue;
                }
                let mut start_x = check_x;
                while let Some(n) = self.schematic.get_loc(start_x, check_y) {
                    if start_x == 0 || !n.is_numeric() {
                        break;
                    }
                    if let Some(n) = self.schematic.get_loc(start_x - 1, check_y) {
                        if n.is_numeric() {
                            start_x -= 1;
                        } else {
                            break;
                        }
                    }
                }
                let mut value = 0;
                while let Some(n) = self.schematic.get_loc(start_x, check_y) {
                    if !n.is_numeric() {
                        break;
                    }

                    self.to_check.retain(|e| {
                        (
                            e.0 + self.centerpoint.0 as isize,
                            e.1 + self.centerpoint.1 as isize,
                        ) != (start_x as isize, check_y as isize)
                    });
                    start_x += 1;
                    value = (value * 10) + n.to_digit(10).unwrap();
                }
                return Some(value as usize);
            }
        }
    }
}

impl Schematic<'_> {
    pub fn new(board: &str) -> Schematic {
        Schematic { board }
    }

    pub fn part_label_sum(&self, x: usize, y: usize) -> usize {
        self.adjacent_numbers(x, y).sum()
    }

    pub fn parts_locations(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.board.lines().enumerate().flat_map(|(j, row)| {
            row.chars().enumerate().filter_map(move |(i, c)| match c {
                '.' => None,
                c if c.is_numeric() => None,
                _ => Some((i, j)),
            })
        })
    }

    pub fn location_by_type(&self, t: char) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.board.lines().enumerate().flat_map(move |(j, row)| {
            row.chars().enumerate().filter_map(move |(i, c)| match c {
                c if c == t => Some((i, j)),
                _ => None,
            })
        })
    }

    fn get_loc(&self, x: usize, y: usize) -> Option<char> {
        self.board.lines().nth(y)?.chars().nth(x)
    }

    pub fn adjacent_numbers(&self, x: usize, y: usize) -> impl Iterator<Item = usize> + '_ {
        AdjacentNumberIterator {
            schematic: self,
            centerpoint: (x, y),
            to_check: [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .to_vec(),
        }
    }

    pub fn gear_ratio(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let v: Vec<_> = self.adjacent_numbers(x, y).collect();
        if v.len() == 2 {
            return Some((v[0], v[1]));
        }
        None
    }
}
