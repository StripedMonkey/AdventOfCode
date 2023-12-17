use core::panic;

use pathfinding::directed::{astar::astar, dijkstra::dijkstra};
use rs_17::*;

fn main() {
    let file = *INPUT_1;
    let map = parse_file(&file);
    let result = map.puzzle();
    println!("The answer is {}", result); // 1155
}

struct HeatMap {
    map: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Hash)]
struct CruciblePath {
    position: (usize, usize),
    direction: Direction,
    previous: usize,
}

impl PartialEq for CruciblePath {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for CruciblePath {}

impl CruciblePath {
    fn new(direction: Direction, x: usize, y: usize) -> CruciblePath {
        CruciblePath {
            position: (x, y),
            direction,
            previous: 1,
        }
    }
    fn cost(&self, map: &HeatMap) -> usize {
        map.get(self.position.0, self.position.1)
    }
    fn successors<'a>(&self, map: &'a HeatMap) -> impl Iterator<Item = (CruciblePath, usize)> + 'a {
        let direction = self.direction;
        let (current_x, current_y) = self.position;
        let mut successors = Vec::new();

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
        'forward: {
            if self.previous >= 3 {
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

    fn puzzle(&self) -> usize {
        let start = CruciblePath::new(Direction::Right, 0, 0);
        let end = CruciblePath::new(Direction::Right, self.width() - 1, self.height() - 1);
        let path = dijkstra(&start, |p| p.successors(&self), |p| *p == end);
        let Some(path) = path else {
            panic!("No Path");
        };
        path.1
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.map[y][x]
    }
}

fn parse_file(file: &str) -> HeatMap {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_17::static_read("example1.txt");
        let map = parse_file(&file);
        let result = map.puzzle();
        assert_eq!(result, 102);
    }
}
