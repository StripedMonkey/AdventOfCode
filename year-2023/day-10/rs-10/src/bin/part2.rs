use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    thread::sleep,
    time::Duration,
    vec,
};

use rs_10::*;

fn main() {
    let file = *INPUT_1;
    let maze = PipeMaze::new(file);
    println!("{}", maze);
    maze.find_enclosed();
    sleep(Duration::from_secs(1)); // vscode term failures
}

struct PipeMaze {
    map: Vec<Vec<char>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Center,
    Top,
    Bottom,
    Left,
    Right,
}

impl Display for PipeMaze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter().rev() {
            for ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position(usize, usize);

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

struct CycleFinder {
    maze: PipeMaze,
    start: Position,
    pipes_visited: HashSet<Position>,
    external_visited: HashSet<Position>,
    ext_pipes_visited: HashSet<(Direction, Position)>,
}

impl Display for CycleFinder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.maze.map.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                let pos = Position(j, self.maze.map.len() - i - 1);
                if self.pipes_visited.contains(&pos) {
                    write!(f, "\x1b[93m{ch}\x1b[0m")?;
                } else {
                    write!(f, "{}", ch)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl CycleFinder {
    fn print_highlight(&self, highlight_pos: Position) {
        use std::fmt::Write;
        let mut map_str = String::new();
        let height_pad = (self.maze.map.len()).checked_ilog10().unwrap() + 1;

        self.maze.map.iter().enumerate().rev().for_each(|(i, row)| {
            write!(map_str, "{:width$} ", i, width = height_pad as usize).unwrap();
            row.iter().enumerate().for_each(|(j, ch)| {
                let pos = Position(j, i);
                if pos == highlight_pos {
                    write!(map_str, "\x1b[92m{ch}\x1b[0m").unwrap();
                } else if self.pipes_visited.contains(&pos) {
                    write!(map_str, "\x1b[93m{ch}\x1b[0m").unwrap();
                } else if self.external_visited.contains(&pos) {
                    write!(map_str, "\x1b[94m{ch}\x1b[0m").unwrap();
                } else {
                    write!(map_str, ".").unwrap();
                }
            });
            writeln!(map_str).unwrap();
        });
        println!("{}", map_str);
    }
    fn external_stack<I>(&self, stack: &mut VecDeque<(Direction, Position)>, pos: Position, it: I)
    where
        I: IntoIterator<Item = (Direction, (isize, isize))>,
    {
        stack.extend(
            it.into_iter()
                .filter_map(|(dir, offset)| {
                    Some((
                        dir,
                        Position(
                            pos.0.checked_add_signed(offset.0)?,
                            pos.1.checked_add_signed(offset.1)?,
                        ),
                    ))
                })
                .filter(|(d, p)| !{
                    // let c = self.maze.get_position(p);
                    // if Position(2, 3) == *p && (*d == Direction::Left) {
                    //     self.print_highlight(*p);
                    //     println!("E")
                    // }
                    self.external_visited.contains(p) || self.ext_pipes_visited.contains(&(*d, *p))
                }),
        )
    }

    fn find_enclosed(&mut self) -> usize {
        let mut stack = VecDeque::new();
        let mut length = 0;
        stack.push_back(self.start);
        while let Some(pos) = stack.pop_back() {
            self.pipes_visited.insert(pos);
            length += 1;
            if let Some(adjacents) = self.maze.mutually_adj(pos) {
                for adj in adjacents {
                    if !self.pipes_visited.contains(&adj) {
                        stack.push_back(adj);
                    }
                }
            }
        }
        let cleaned_map = self.cleaned_map();
        self.maze = PipeMaze::new_raw(&cleaned_map);

        let mut flood_stack = VecDeque::new();
        flood_stack.push_back((Direction::Center, Position(0, 0)));

        while let Some((approach_dir, position)) = flood_stack.pop_back() {
            if self.external_visited.contains(&position) {
                continue;
            }
            let Some(ch) = self.maze.get_position(&position) else {
                continue;
            };
            if ch == '.' {
                self.external_visited.insert(position);
                self.external_stack(
                    &mut flood_stack,
                    position,
                    vec![
                        (Direction::Top, (0, -1)),
                        (Direction::Bottom, (0, 1)),
                        (Direction::Right, (-1, 0)),
                        (Direction::Left, (1, 0)),
                    ],
                );
                continue;
            }
            self.ext_pipes_visited.insert((approach_dir, position));
            // self.print_highlight(position);
            let next = match ch {
                '|' => match approach_dir {
                    Direction::Left => vec![
                        (Direction::Left, (0, 1)),
                        (Direction::Left, (0, -1)),
                        (Direction::Right, (-1, 0)),
                    ],
                    Direction::Right => vec![
                        (Direction::Right, (0, 1)),
                        (Direction::Right, (0, -1)),
                        (Direction::Left, (1, 0)),
                    ],
                    _ => vec![],
                },
                '-' => match approach_dir {
                    Direction::Top => vec![
                        (Direction::Top, (1, 0)),
                        (Direction::Top, (-1, 0)),
                        (Direction::Bottom, (0, 1)),
                    ],
                    Direction::Bottom => vec![
                        (Direction::Bottom, (1, 0)),
                        (Direction::Bottom, (-1, 0)),
                        (Direction::Top, (0, -1)),
                    ],
                    _ => vec![],
                },
                'L' => match approach_dir {
                    Direction::Top => vec![(Direction::Right, (0, 1)), (Direction::Top, (1, 0))],
                    Direction::Bottom => vec![
                        (Direction::Left, (0, 1)),
                        (Direction::Right, (-1, 0)),
                        (Direction::Bottom, (1, 0)),
                        (Direction::Top, (0, -1)),
                    ],
                    Direction::Left => vec![
                        (Direction::Left, (0, 1)),
                        (Direction::Right, (-1, 0)),
                        (Direction::Bottom, (1, 0)),
                        (Direction::Top, (0, -1)),
                    ],
                    Direction::Right => vec![(Direction::Right, (0, 1)), (Direction::Top, (1, 0))],
                    _ => vec![],
                },
                'J' => match approach_dir {
                    Direction::Top => vec![(Direction::Left, (0, 1)), (Direction::Top, (-1, 0))],
                    Direction::Bottom => vec![
                        (Direction::Right, (0, 1)),
                        (Direction::Bottom, (-1, 0)),
                        (Direction::Left, (1, 0)),
                        (Direction::Top, (0, -1)),
                    ],
                    Direction::Left => vec![(Direction::Left, (0, 1)), (Direction::Top, (-1, 0))],
                    Direction::Right => vec![
                        (Direction::Right, (0, 1)),
                        (Direction::Bottom, (-1, 0)),
                        (Direction::Left, (1, 0)),
                        (Direction::Top, (0, -1)),
                    ],
                    _ => vec![],
                },
                '7' => match approach_dir {
                    Direction::Top => vec![
                        (Direction::Bottom, (0, 1)),
                        (Direction::Top, (-1, 0)),
                        (Direction::Left, (1, 0)),
                        (Direction::Right, (0, -1)),
                    ],
                    Direction::Bottom => {
                        vec![
                            (Direction::Bottom, (-1, 0)), 
                            (Direction::Left, (0, -1))]
                    }
                    Direction::Left => {
                        vec![(Direction::Bottom, (-1, 0)), (Direction::Left, (0,-1))]
                    }
                    Direction::Right => vec![
                        (Direction::Bottom, (0, 1)),
                        (Direction::Top, (-1, 0)),
                        (Direction::Left, (1, 0)),
                        (Direction::Right, (0, -1)),
                    ],
                    _ => vec![],
                },
                'F' => match approach_dir {
                    Direction::Top => vec![
                        (Direction::Bottom, (0, 1)),
                        (Direction::Right, (-1, 0)),
                        (Direction::Top, (1, 0)),
                        (Direction::Left, (0, -1)),
                    ],
                    Direction::Bottom => {
                        vec![
                            (Direction::Bottom, (1, 0)), 
                            (Direction::Right, (0, -1))
                        ]
                    }
                    Direction::Left => vec![
                        (Direction::Bottom, (0, 1)),
                        (Direction::Right, (-1, 0)),
                        (Direction::Top, (1, 0)),
                        (Direction::Left, (0, -1)),
                    ],
                    Direction::Right => {
                        vec![(Direction::Bottom, (1, 0)), (Direction::Right, (0, -1))]
                    }
                    _ => vec![],
                },
                'S' => vec![],
                c => {
                    println!("Unexpected Character {c} at {position}");
                    vec![]
                }
            };
            self.external_stack(&mut flood_stack, position, next);
        }

        println!("Length: {length}");
        println!("Num Flooded: {}", self.external_visited.len());
        let (x, img) = self.cleaned_flood();
        println!("{}", img);
        println!("External tiles: {}", self.external_visited.len());
        // println!("External tiles: {:?}", self.external_visited);
        println!(
            "Maybe internal: {}, {}",
            x,
            (self.external_visited.len() + length)
                .abs_diff(self.maze.map.iter().map(|x| x.len()).sum())
        );
        println!("Maybe Furthest: {}", (length) / 2);
        // println!("{:?}", self.edges);
        x
    }

    fn cleaned_flood(&self) -> (usize, String) {
        use std::fmt::Write;
        let mut num_internal = 0;
        let mut map_str = String::new();
        self.maze.map.iter().enumerate().rev().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, ch)| {
                let pos = Position(j, i);
                if self.pipes_visited.contains(&pos) {
                    write!(map_str, "\x1b[93m{ch}\x1b[0m").unwrap();
                } else if self.external_visited.contains(&pos) {
                    write!(map_str, "\x1b[94m{ch}\x1b[0m").unwrap();
                } else {
                    write!(map_str, ".").unwrap();
                    num_internal += 1;
                }
            });
            writeln!(map_str).unwrap();
        });
        (num_internal, map_str)
    }

    fn cleaned_map(&self) -> String {
        use std::fmt::Write;
        let mut map_str = String::new();
        self.maze.map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, ch)| {
                let pos = Position(j, i);
                if self.pipes_visited.contains(&pos) {
                    write!(map_str, "{ch}").unwrap();
                } else {
                    write!(map_str, ".").unwrap();
                }
            });
            writeln!(map_str).unwrap();
        });
        map_str
    }
}

impl PipeMaze {
    fn new(file: &str) -> Self {
        let map = file
            .lines()
            .rev()
            .map(|line| line.chars().collect())
            .collect();
        PipeMaze { map }
    }

    fn new_raw(file: &str) -> Self {
        let map = file.lines().map(|line| line.chars().collect()).collect();
        PipeMaze { map }
    }

    fn get_position(&self, Position(x, y): &Position) -> Option<char> {
        self.map.get(*y).and_then(|row| row.get(*x).copied())
    }

    fn starting_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.map
            .iter()
            .enumerate()
            .map(move |(y, row)| {
                row.iter().enumerate().filter_map(move |(x, c)| {
                    if *c == 'S' {
                        println!("Starting Position: {x},{y}");
                        Some(Position(x, self.map.len() - y - 1))
                    } else {
                        None
                    }
                })
            })
            .flatten()
    }

    fn mutually_adj(&self, current_pos: Position) -> Option<Vec<Position>> {
        let adj = self.adjacent(current_pos)?;
        Some(
            adj.iter()
                .filter_map(|pos| {
                    let other_adj = self.adjacent(*pos)?;
                    if other_adj.contains(&current_pos) {
                        return Some(*pos);
                    }
                    None
                })
                .collect(),
        )
    }

    fn adjacent(&self, pos: Position) -> Option<Vec<Position>> {
        let ch = self.get_position(&pos);
        let Position(x, y) = pos;

        let immediate_adjs = match ch {
            Some('|') => vec![(0, -1), (0, 1)],
            Some('-') => vec![(-1, 0), (1, 0)],
            Some('L') => vec![(1, 0), (0, 1)],
            Some('J') => vec![(-1, 0), (0, 1)],
            Some('7') => vec![(-1, 0), (0, -1)],
            Some('F') => vec![(1, 0), (0, -1)],
            Some('S') => vec![(0, -1), (0, 1), (-1, 0), (1, 0)],
            Some('.') => return None,
            _ => panic!(),
        };
        Some(
            immediate_adjs
                .iter()
                .filter_map(|pos| {
                    let pos = Position(x.checked_add_signed(pos.0)?, y.checked_add_signed(pos.1)?);
                    if let Some(c) = self.get_position(&pos) {
                        if c != '.' {
                            return Some(pos);
                        }
                    }
                    None
                })
                .collect(),
        )
    }

    fn find_enclosed(self) -> usize {
        let start = self.starting_positions().next().unwrap();
        let mut finder = CycleFinder {
            maze: self,
            start,
            pipes_visited: HashSet::new(),
            external_visited: HashSet::new(),
            ext_pipes_visited: HashSet::new(),
        };
        finder.find_enclosed()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_10::static_read("example2.txt");
        let maze = PipeMaze::new(file);
        println!("{}", maze);
        let result = maze.find_enclosed();
        assert_eq!(result, 4);
    }

    #[test]
    fn second_test() {
        let file = rs_10::static_read("example3.txt");
        let maze = PipeMaze::new(file);
        println!("{}", maze);
        let result = maze.find_enclosed();
        assert_eq!(result, 4);

    }

    #[test]
    fn third_test() {
        let file = rs_10::static_read("example4.txt");
        let maze = PipeMaze::new(file);
        println!("{}", maze);
        let result = maze.find_enclosed();
        assert_eq!(result, 15);
    }

    #[test]
    fn fourth_test() {
        let file = rs_10::static_read("example5.txt");
        let maze = PipeMaze::new(file);
        println!("{}", maze);
        let result = maze.find_enclosed();
        assert_eq!(result, 11);
    }

    #[test]
    fn fifth_test() {
        let file = rs_10::static_read("example6.txt");
        let maze = PipeMaze::new(file);
        println!("{}", maze);
        let result = maze.find_enclosed();
        assert_eq!(result, 13);
    }

    #[test]
    fn sixth_test() {
        let file = rs_10::static_read("example7.txt");
        let maze = PipeMaze::new(file);
        println!("{}", maze);
        let result = maze.find_enclosed();
        assert_eq!(result, 13);
    }

    #[test]
    #[ignore = "Inputs are not included and vary by person"]
    fn input_test() {
        let file = rs_10::static_read("input1.txt");
        let maze = PipeMaze::new(file);
        println!("{}", maze);
        let result = maze.find_enclosed();
        assert_eq!(result, 459);
    }
}
