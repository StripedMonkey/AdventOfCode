use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use rs_10::*;

fn main() {
    let file = *INPUT_1;
    let maze = PipeMaze::new(file);
    println!("{}", maze);
    maze.find_cycle();
}

struct PipeMaze {
    map: Vec<Vec<char>>,
}

impl Display for PipeMaze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
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

struct CycleFinder<'a> {
    maze: &'a PipeMaze,
    start: Position,
    visited: HashSet<Position>,
    edges: HashSet<(Position, Position)>,
}

impl Display for CycleFinder<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.maze.map.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                let pos = Position(j, self.maze.map.len() - i);
                if self.visited.contains(&pos) {
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

impl CycleFinder<'_> {
    fn find_cycle(&mut self) {
        let mut stack = VecDeque::new();
        let mut length = 0;
        stack.push_back(self.start);
        while let Some(pos) = stack.pop_back() {
            self.visited.insert(pos);
            length += 1;
            if let Some(adjacents) = self.maze.mutually_adj(pos) {
                // println!("Visiting Position {pos:?}, Adjacents: {:?}", adjacents);
                for adj in adjacents {
                    if self.visited.contains(&adj) {
                    } else {
                        stack.push_back(adj);
                    }
                }
            }
        }
        println!("Length: {length}");
        println!("{}", self);
        println!("Maybe Furthest: {}", (length) / 2);
        // println!("{:?}", self.edges);
    }
}

impl PipeMaze {
    fn new(file: &str) -> Self {
        let map = file.lines().map(|line| line.chars().collect()).collect();
        PipeMaze { map }
    }

    fn get_position(&self, Position(x, y): &Position) -> Option<char> {
        self.map
            .get(self.map.len() - y)
            .and_then(|row| row.get(*x).copied())
    }

    fn starting_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.map
            .iter()
            .enumerate()
            .map(move |(y, row)| {
                row.iter().enumerate().filter_map(move |(x, c)| {
                    if *c == 'S' {
                        println!("Starting Position: {x},{y}");
                        Some(Position(x, self.map.len() - y))
                    } else {
                        None
                    }
                })
            })
            .flatten()
    }

    fn mutually_adj(&self, current_pos: Position) -> Option<Vec<Position>> {
        let adj = self.adjacent(current_pos)?;
        Some(adj.iter().filter_map(|pos| {
            let other_adj = self.adjacent(*pos)?;
            if other_adj.contains(&current_pos) {
                return Some(*pos);
            }
            None
        }).collect())
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

    fn find_cycle(&self) {
        let mut finder = CycleFinder {
            maze: self,
            start: self.starting_positions().next().unwrap(),
            visited: HashSet::new(),
            edges: HashSet::new(),
        };
        finder.find_cycle();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_10::static_read("example1.txt");
        let maze = PipeMaze::new(file);
        println!("{}", maze);
        maze.find_cycle();
    }
}
