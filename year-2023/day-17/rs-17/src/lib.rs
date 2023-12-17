use std::{path::PathBuf, str::FromStr};

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

use pathfinding::directed::dijkstra::dijkstra;

pub struct HeatMap {
    map: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct CruciblePath<const MIN_TURN: usize, const MAX_TURN: usize> {
    position: (usize, usize),
    direction: Direction,
    previous: usize,
}

impl<const MIN_TURN: usize, const MAX_TURN: usize> CruciblePath<MIN_TURN, MAX_TURN> {
    fn new(direction: Direction, x: usize, y: usize) -> CruciblePath<MIN_TURN, MAX_TURN> {
        CruciblePath {
            position: (x, y),
            direction,
            previous: 0,
        }
    }

    fn cost(&self, map: &HeatMap) -> usize {
        map.get(self.position.0, self.position.1)
    }

    fn successors<'a>(
        &self,
        map: &'a HeatMap,
    ) -> impl Iterator<Item = (CruciblePath<MIN_TURN, MAX_TURN>, usize)> + 'a {
        let direction = self.direction;
        let (current_x, current_y) = self.position;
        let mut successors = Vec::new();

        if self.previous >= MIN_TURN {
            match direction {
                Direction::Up | Direction::Down => {
                    if current_x > 0 {
                        successors.push(CruciblePath {
                            position: (current_x - 1, current_y),
                            direction: Direction::Left,
                            previous: 1,
                        });
                    }
                    if current_x < map.width() - 1 {
                        successors.push(CruciblePath {
                            position: (current_x + 1, current_y),
                            direction: Direction::Right,
                            previous: 1,
                        });
                    }
                }
                Direction::Left | Direction::Right => {
                    if current_y > 0 {
                        successors.push(CruciblePath {
                            position: (current_x, current_y - 1),
                            direction: Direction::Up,
                            previous: 1,
                        });
                    }
                    if current_y < map.height() - 1 {
                        successors.push(CruciblePath {
                            position: (current_x, current_y + 1),
                            direction: Direction::Down,
                            previous: 1,
                        });
                    }
                }
            }
        }

        'forward: { // for the break 'forward
            if self.previous >= MAX_TURN {
                break 'forward;
            }
            successors.push(CruciblePath {
                position: {
                    let (x, y) = self.position;
                    match direction {
                        Direction::Up if y > 0 => (x, y - 1),
                        Direction::Down if y < map.height() - 1 => (x, y + 1),
                        Direction::Left if x > 0 => (x - 1, y),
                        Direction::Right if x < map.width() - 1 => (x + 1, y),
                        _ => break 'forward,
                    }
                },
                direction,
                previous: self.previous + 1,
            })
        }
        successors.into_iter().map(move |p| {
            let cost = p.cost(&map);
            (p, cost)
        })
    }
}

impl HeatMap {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    pub fn puzzle<const MIN: usize, const MAX: usize>(&self) -> usize {
        let right_start = CruciblePath::<MIN, MAX>::new(Direction::Right, 0, 0);
        let right_path = dijkstra(
            &right_start,
            |p| p.successors(&self),
            |p| p.position == (self.width() - 1, self.height() - 1) && p.previous >= MIN,
        );
        let down_start = CruciblePath::<MIN, MAX>::new(Direction::Down, 0, 0);
        let down_path = dijkstra(
            &down_start,
            |p| p.successors(&self),
            |p| p.position == (self.width() - 1, self.height() - 1) && p.previous >= MIN,
        );
        let path = match (right_path, down_path) {
            (None, None) => None,
            (Some(right), None) => Some(right),
            (None, Some(down)) => Some(down),
            (Some(right), Some(down)) => {
                if right.1 < down.1 {
                    Some(right)
                } else {
                    Some(down)
                }
            }
        };

        let Some(path) = path else {
            panic!("No Path");
        };
        // self.print_map_highlight(path.0.iter().map(|p| p.position).collect_vec());
        path.1
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.map[y][x]
    }

    fn print_map_highlight(&self, highlights: Vec<(usize, usize)>) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if highlights.contains(&(x, y)) {
                    print!("\x1b[31m{}\x1b[0m", cell);
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
    }
}

pub fn parse_file(file: &str) -> HeatMap {
    let map = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    HeatMap { map }
}
