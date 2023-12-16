use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
    str::FromStr,
};

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

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct BeamMap {
    map: Vec<Vec<char>>,
}

fn print_map<F>(map: &BeamMap, highlight: F)
where
    F: Fn((usize, usize)) -> bool,
{
    println!("Map:");
    for (y, row) in map.map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if highlight((x, y)) {
                print!("X");
            } else {
                print!("{c}");
            }
        }
        println!();
    }
}

impl BeamMap {
    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.map.get(y).and_then(|row| row.get(x).copied())
    }
    pub fn height(&self) -> usize {
        self.map.len()
    }
    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn traverse(&self, start: (Direction, (usize, usize))) -> usize {
        let mut set = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some((direction, (x, y))) = queue.pop_front() {
            let Some(c) = self.get(x, y) else {
                continue;
            };
            if !set.insert((direction, (x, y))) {
                continue;
            };
            // if c != '.' {
            //     println!("Direction: {direction:?}, ({x},{y}), {c:?}");
            //     print_map(&self, |p| set.iter().any(|(_, p2)| p == *p2));
            // }
            match c {
                '.' => {
                    let (x, y) = match direction {
                        Direction::Up => {
                            let Some(y) = y.checked_sub(1) else {
                                continue;
                            };
                            (x, y)
                        }
                        Direction::Down => (x, y + 1),
                        Direction::Left => {
                            let Some(x) = x.checked_sub(1) else {
                                continue;
                            };
                            (x, y)
                        }
                        Direction::Right => (x + 1, y),
                    };
                    queue.push_back((direction, (x, y)));
                }
                '\\' => {
                    let (dir, (x, y)) = match direction {
                        Direction::Up => {
                            let Some(x) = x.checked_sub(1) else {
                                continue;
                            };
                            (Direction::Left, (x, y))
                        }
                        Direction::Down => (Direction::Right, (x + 1, y)),
                        Direction::Left => {
                            let Some(y) = y.checked_sub(1) else {
                                continue;
                            };
                            (Direction::Up, (x, y))
                        }
                        Direction::Right => (Direction::Down, (x, y + 1)),
                    };
                    queue.push_back((dir, (x, y)));
                }
                '/' => {
                    let (dir, (x, y)) = match direction {
                        Direction::Up => (Direction::Right, (x + 1, y)),
                        Direction::Down => {
                            let Some(x) = x.checked_sub(1) else {
                                continue;
                            };
                            (Direction::Left, (x, y))
                        }
                        Direction::Left => (Direction::Down, (x, y + 1)),
                        Direction::Right => {
                            let Some(y) = y.checked_sub(1) else {
                                continue;
                            };
                            (Direction::Up, (x, y))
                        }
                    };
                    queue.push_back((dir, (x, y)));
                }
                '|' => match direction {
                    Direction::Left | Direction::Right => {
                        if let Some(y) = y.checked_sub(1) {
                            queue.push_back((Direction::Up, (x, y)));
                        };
                        queue.push_back((Direction::Down, (x, y + 1)));
                    }
                    Direction::Up => {
                        if let Some(y) = y.checked_sub(1) {
                            queue.push_back((Direction::Up, (x, y)));
                        };
                    }
                    Direction::Down => {
                        queue.push_back((Direction::Down, (x, y + 1)));
                    }
                },
                '-' => match direction {
                    Direction::Up | Direction::Down => {
                        if let Some(x) = x.checked_sub(1) {
                            queue.push_back((Direction::Left, (x, y)));
                        };
                        queue.push_back((Direction::Right, (x + 1, y)));
                    }
                    Direction::Left => {
                        if let Some(x) = x.checked_sub(1) {
                            queue.push_back((Direction::Left, (x, y)));
                        };
                    }
                    Direction::Right => {
                        queue.push_back((Direction::Right, (x + 1, y)));
                    }
                },
                c => panic!("Unknown character {c}"),
            }
        }
        HashSet::<(usize, usize)>::from_iter(set.iter().map(|(_, p)| *p)).len()
    }
}

pub fn parse_file(file: &str) -> BeamMap {
    let map = file.lines().map(|line| line.chars().collect()).collect();
    BeamMap { map }
}
