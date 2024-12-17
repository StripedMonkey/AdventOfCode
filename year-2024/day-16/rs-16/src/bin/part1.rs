use std::{collections::HashSet, iter};

use aoc_utils::*;

use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use rs_2024_16::*;

fn main() {
    let input = rs_2024_16::static_read("input1.txt");
    let (start, end, map) = parse(&input);
    let start = Node {
        position: start,
        facing: Facing::East,
    };
    let dij = dijkstra(&start, |n| n.adjacent(&map), |n| n.get_c(&map) == 'E').unwrap();
    println!("{:?}", dij);
    let answer = dij.1;
    for (y, row) in map.iter().enumerate() {
        println!(
            "{}",
            row.iter()
                .enumerate()
                .map(|(x, c)| {
                    if (x, y) == start.position {
                        'S'
                    } else if (x, y) == end {
                        'E'
                    } else if dij.0.iter().any(|n| n.position == (x, y)) {
                        'O'
                    } else {
                        *c
                    }
                })
                .join("")
        );
    }

    println!("{:?}", answer);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use pathfinding::prelude::dijkstra;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_16::static_read("example2.txt");
        let (start, end, map) = parse(&input);
        let start = Node {
            position: start,
            facing: Facing::East,
        };
        let all_paths = all_minimum_paths(&map, start, end);
        for (y, row) in map.iter().enumerate() {
            println!(
                "{}",
                row.iter()
                    .enumerate()
                    .map(|(x, c)| {
                        if (x, y) == start.position {
                            'S'
                        } else if (x, y) == end {
                            'E'
                        } else if all_paths.contains(&(x, y)) {
                            'O'
                        } else {
                            *c
                        }
                    })
                    .join("")
            );
        }
        let answer = all_paths.len();
        println!("{:?}", answer);
    }
}

fn all_minimum_paths(
    map: &Vec<Vec<char>>,
    start: Node,
    end: (usize, usize),
) -> HashSet<(usize, usize)> {
    let original_dij = dijkstra(&start, |n| n.adjacent(&map), |n| n.position == end).unwrap();
    let mut all_paths = HashSet::new();
    all_paths.extend(original_dij.0.iter().map(|n| n.position));
    let original_cost = original_dij.1;
    loop {
        let mut new_nodes = HashSet::new();
        for node in all_paths.iter() {
            let position = *node;
            let new_dij: (_, usize) = dijkstra(
                &start,
                |n| {
                    n.adjacent(&map).into_iter().map(|e| {
                        if e.0.position == position {
                            (e.0, e.1 + 1)
                        } else {
                            e
                        }
                    })
                },
                |n| n.position == end,
            )
            .unwrap();
            if new_dij.1 == original_cost {
                new_nodes.extend(new_dij.0.iter().map(|n| n.position));
            }
        }
        if new_nodes.is_subset(&all_paths) {
            break;
        }
        all_paths.extend(new_nodes);
    }
    all_paths
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Node {
    position: (usize, usize),
    facing: Facing,
}

impl Node {
    fn get_c(&self, map: &Vec<Vec<char>>) -> char {
        map[self.position.1][self.position.0]
    }
    fn adjacent(&self, map: &Vec<Vec<char>>) -> Vec<(Node, usize)> {
        let mut ans: Vec<(Node, usize)> = self
            .facing
            .adjacent()
            .iter()
            .map(|&f| (Node { facing: f, ..*self }, 1000))
            .collect();
        if let Some(n) = self.forward(map) {
            ans.push(n);
        }
        ans
    }

    fn forward(&self, map: &Vec<Vec<char>>) -> Option<(Node, usize)> {
        let forwards = match self.facing {
            Facing::North => (0, -1),
            Facing::South => (0, 1),
            Facing::East => (1, 0),
            Facing::West => (-1, 0),
        };
        let new_pos = (
            self.position.0 as isize + forwards.0,
            self.position.1 as isize + forwards.1,
        );
        if new_pos.0 < 0 || new_pos.1 < 0 {
            return None;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        if new_pos.0 >= map[0].len() || new_pos.1 >= map.len() {
            return None;
        }
        if map[new_pos.1][new_pos.0] == '#' {
            return None;
        }
        Some((
            Node {
                position: new_pos,
                facing: self.facing,
            },
            1,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn adjacent(&self) -> [Facing; 2] {
        match self {
            Facing::North | Facing::South => [Facing::East, Facing::West],
            Facing::East | Facing::West => [Facing::North, Facing::South],
        }
    }
}

fn parse(input: &str) -> ((usize, usize), (usize, usize), Vec<Vec<char>>) {
    let mut map: Vec<Vec<_>> = input.lines().map(|c| c.chars().collect()).collect();

    let start = map
        .iter()
        .find_position(|row| row.contains(&'S'))
        .map(|(i, r)| (r.iter().position(|&c| c == 'S').unwrap(), i))
        .unwrap();
    let end = map
        .iter()
        .find_position(|row| row.contains(&'E'))
        .map(|(i, r)| (r.iter().position(|&c| c == 'E').unwrap(), i))
        .unwrap();
    (start, end, map)
}
