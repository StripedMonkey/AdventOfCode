use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::AddAssign as _,
};

use aoc_utils::*;

use itertools::Itertools as _;
use pathfinding::prelude::dijkstra;
use rs_2024_20::*;

fn main() {
    let input = rs_2024_20::static_read("input1.txt");
    let map = parse(input);
    let start = find_c(&map, 'S').unwrap();
    let end = find_c(&map, 'E').unwrap();
    let (base_path, _cost) = dijkstra(
        &Node::new(start, 0, 0),
        |node| node.adjacent(&map).map(|n| (n, 1)),
        |node| node.position == end,
    )
    .unwrap();
    let base_time = base_path.last().unwrap().time;

    let mut bag = HashMap::new();
    for (i, start) in base_path.iter().enumerate() {
        for ending_node in &base_path[i..] {
            if start == ending_node {
                continue;
            }
            let shortcut_length = start.manhattan(&ending_node);
            if shortcut_length > 20 {
                continue;
            }
            let saved_time = ending_node.time as isize - (start.time + shortcut_length) as isize;
            if saved_time <= 0 {
                continue;
            }
            // println!("{:?} -> {:?} ({:?})", start.position, ending_node.position, saved_time);
            // map_print(&map, |pos|{
            //     if pos == start.position {
            //         return Some('A');
            //     }
            //     if pos == ending_node.position {
            //         return Some('B');
            //     }
            //     None
            // });
            let run_time: usize = base_time - saved_time as usize;
            bag.entry(run_time).or_insert(0).add_assign(1);
        }
    }
    let min_saved = 100;
    let entries: Vec<(&_, &_)> = bag
        .iter()
        .filter(|(&t, _)| base_time - t >= min_saved)
        .sorted_by_key(|c| base_time - c.0)
        .collect();
    for (time, n) in entries {
        println!("{} save {}ps", n, base_time - time);
    }
    // entries.iter().unique_by(|(p,_)| p).
    println!(
        "{:?} cheats that save time",
        bag.iter()
            .filter(|(&t, _)| base_time - t >= min_saved)
            .map(|(_, n)| n)
            .sum::<usize>()
    )
}

#[cfg(test)]
mod test {
    use std::{
        collections::{HashMap, HashSet},
        ops::AddAssign,
    };

    use itertools::Itertools;
    use pathfinding::prelude::dijkstra;

    use super::*;

    #[test]
    fn day_20_first_test() {
        let input = rs_2024_20::static_read("example1.txt");
        let map = parse(input);
        let start = find_c(&map, 'S').unwrap();
        let end = find_c(&map, 'E').unwrap();
        let (base_path, _cost) = dijkstra(
            &Node::new(start, 0, 0),
            |node| node.adjacent(&map).map(|n| (n, 1)),
            |node| node.position == end,
        )
        .unwrap();
        let base_time = base_path.last().unwrap().time;

        let mut bag = HashMap::new();
        for start in &base_path {
            for ending_node in &base_path {
                if start == ending_node {
                    continue;
                }
                let shortcut_length = start.manhattan(&ending_node);
                if shortcut_length > 2 {
                    continue;
                }
                let saved_time =
                    ending_node.time as isize - (start.time + shortcut_length) as isize;
                println!(
                    "{:?} -> {:?} ({:?})",
                    start.position, ending_node.position, saved_time
                );
                map_print(&map, |pos| {
                    if pos == start.position {
                        return Some('H');
                    }
                    if pos == ending_node.position {
                        return Some('T');
                    }
                    None
                });
                if saved_time > 0 {
                    bag.entry(ending_node.time).or_insert(0).add_assign(1);
                }
            }
        }
        let min_saved = 50;
        let entries: Vec<(&usize, &usize)> = bag
            .iter()
            .filter(|(&t, _)| base_time - t >= min_saved)
            .sorted_by_key(|c| base_time - c.0)
            .collect();
        for (time, n) in entries {
            println!("{} save {}ps", n, base_time - time);
        }
        // entries.iter().unique_by(|(p,_)| p).
        println!(
            "{:?} cheats that save time",
            bag.iter()
                .filter(|(&t, _)| base_time - t >= min_saved)
                .map(|(_, n)| n)
                .sum::<usize>()
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    position: (usize, usize),
    time: usize,
    cheat_time_remaining: usize,
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Node {}

impl Node {
    fn new(position: (usize, usize), steps: usize, ignorable_walls: usize) -> Self {
        Self {
            position,
            time: steps,
            cheat_time_remaining: ignorable_walls,
        }
    }

    fn adjacent<'a>(&self, map: &'a Vec<Vec<char>>) -> impl Iterator<Item = Node> + 'a {
        let pos = self.position;
        let steps = self.time;
        let cheat_time_remaining = self.cheat_time_remaining;
        adjacent_on_map(map, pos)
            .filter(move |&(x, y)| cheat_time_remaining > 1 || map[y][x] != '#')
            .map(move |position| {
                Node::new(position, steps + 1, cheat_time_remaining.saturating_sub(1))
            })
    }

    fn adjacent_walls<'a>(&self, map: &'a Vec<Vec<char>>) -> impl Iterator<Item = Node> + 'a {
        self.adjacent(map).filter(|n| n.on(map) == '#')
    }

    fn manhattan(&self, other: &Node) -> usize {
        let (x1, y1) = self.position;
        let (x2, y2) = other.position;
        x1.abs_diff(x2) + y1.abs_diff(y2)
    }

    fn on(&self, map: &Vec<Vec<char>>) -> char {
        map[self.position.1][self.position.0]
    }

    fn hack_reachable<'a>(&self, map: &'a Vec<Vec<char>>) -> impl Iterator<Item = Node> + 'a {
        let can = self.cheat_time_remaining;
        self.adjacent(map).take_while(move |_| can > 0)
    }
}

fn print_map(
    map: &Vec<Vec<char>>,
    path: &(Vec<Node>, usize),
    hack_path: Vec<Node>,
    hack_start: (usize, usize),
) {
    map_print(map, |(x, y)| {
        if path.0.iter().any(|n| n.position == (x, y)) {
            return Some('O');
        }
        if (x, y) == hack_start {
            return Some('C');
        }
        None
    });
}

fn map_print(map: &Vec<Vec<char>>, F: impl Fn((usize, usize)) -> Option<char>) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Some(c) = F((x, y)) {
                print!("{}", c);
                continue;
            }
            print!("{}", cell);
        }
        println!();
    }
}

fn adjacent_on_map(
    map: &Vec<Vec<char>>,
    (x, y): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    directions
        .into_iter()
        .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .filter(|(x, y)| x < &map[0].len() && y < &map.len())
}

fn find_end(map: &Vec<Vec<char>>, start: Node, end: (usize, usize)) -> Option<(Vec<Node>, usize)> {
    dijkstra(
        &start,
        |node| node.adjacent(&map).map(|n| (n, 1)),
        |node| node.position == end,
    )
    // .map(|(path, cost)| (path.last().unwrap().clone(), cost))
}

fn find_c(map: &Vec<Vec<char>>, c: char) -> Option<(usize, usize)> {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == c {
                return Some((x, y));
            }
        }
    }
    None
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
