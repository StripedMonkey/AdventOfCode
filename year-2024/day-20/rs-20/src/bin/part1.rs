use std::{cmp::Reverse, collections::HashMap, hash::Hash, ops::AddAssign as _};

use aoc_utils::*;

use itertools::Itertools as _;
use pathfinding::prelude::dijkstra;
use rs_2024_20::*;

fn main() {
    let input = rs_2024_20::static_read("input1.txt");
    let map = parse(input);
    let start = find_c(&map, 'S').unwrap();
    let end = find_c(&map, 'E').unwrap();
    let base_path = dijkstra(
        &Node::new(start, 0, 0),
        |node| node.adjacent(&map).map(|n| (n, 1)),
        |node| node.position == end,
    )
    .unwrap();
    // print_map(&map, &base_path, 0, 0);
    // assert_eq!(base_path.1, 84);

    let mut bag = HashMap::new();
    let mut entries = Vec::new();
    for hack_start in base_path.0 {
        let mut starting_node = hack_start.clone();
        starting_node.ignorable_walls = 1;
        for node in starting_node.adjacent(&map) {
            let Some(path) = dijkstra(
                &node,
                |node| node.adjacent(&map).map(|n| (n, 1)),
                |node| node.position == end,
            ) else {
                continue;
            };
            let path_length = path.0.last().unwrap().steps;
            if base_path.1 <= path_length {
                continue;
            }
            // print_map(&map, &path, 0, 1);
            // println!("{}ps", base_path.1 - path_length);
            entries.push((starting_node.position, path_length));
            bag.entry(path_length).or_insert(0).add_assign(1);
        }
    }
    for (time, n) in bag
        .iter()
        .filter(|(&t, _)| base_path.1 - t >= 100)
        .sorted_by_key(|c| base_path.1 - c.0)
    {
        println!("{} save {}ps", n, base_path.1 - time);
    }
    // entries.iter().unique_by(|(p,_)| p).
    let answer = bag.iter()
    .filter(|(&t, _)| base_path.1 - t >= 100)
    .map(|(_, n)| n)
    .sum::<usize>();
    println!(
        "{:?} cheats that save time",
        answer
    );
    assert!(answer == 1351);
}

#[cfg(test)]
mod test {
    use std::{cmp::Reverse, collections::HashMap, ops::AddAssign};

    use itertools::Itertools;
    use pathfinding::prelude::dijkstra;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_20::static_read("example1.txt");
        let map = parse(input);
        let start = find_c(&map, 'S').unwrap();
        let end = find_c(&map, 'E').unwrap();
        let base_path = dijkstra(
            &Node::new(start, 0, 0),
            |node| node.adjacent(&map).map(|n| (n, 1)),
            |node| node.position == end,
        )
        .unwrap();
        print_map(&map, &base_path, 0, 0);
        assert_eq!(base_path.1, 84);

        let mut bag = HashMap::new();
        for hack_start in 0..base_path.0.len() {
            let mut starting_node = base_path.0.get(hack_start).unwrap().clone();
            starting_node.ignorable_walls = 1;
            for node in starting_node.adjacent(&map) {
                let Some(path) = dijkstra(
                    &node,
                    |node| node.adjacent(&map).map(|n| (n, 1)),
                    |node| node.position == end,
                ) else {
                    continue;
                };
                let path_length = path.0.last().unwrap().steps;
                if base_path.1 <= path_length {
                    continue;
                }
                print_map(&map, &path, 0, 1);
                println!("{}ps", base_path.1 - path_length);
                bag.entry(path_length).or_insert(0).add_assign(1);
            }
        }
        for (time, n) in bag.iter().sorted_by_key(|c| base_path.1 - c.0) {
            println!("{} save {}ps", n, base_path.1 - time);
        }
        println!(
            "{:?} cheats that save time",
            bag.iter()
                .filter(|(&t, _)| t > 0)
                .map(|(_, n)| n)
                .sum::<usize>()
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    position: (usize, usize),
    steps: usize,
    ignorable_walls: usize,
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
            steps,
            ignorable_walls,
        }
    }

    fn adjacent<'a>(&self, map: &'a Vec<Vec<char>>) -> impl Iterator<Item = Node> + 'a {
        let should_clip = self.ignorable_walls != 0;
        let pos = self.position;
        let steps = self.steps;
        let ignorable_walls = self.ignorable_walls.saturating_sub(1);
        adjacent(map, pos, should_clip)
            .map(move |position| Node::new(position, steps + 1, ignorable_walls))
    }
}

fn print_map(map: &Vec<Vec<char>>, path: &(Vec<Node>, usize), hack_start: usize, hack_end: usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Some(p) = path.0.iter().find(|p| p.position == (x, y)) {
                if (hack_start + 1..hack_start + 1 + 2).contains(&p.steps) {
                    print!("{}", p.steps - hack_start)
                } else {
                    print!(".");
                }
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}

fn adjacent(
    map: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    ignore_walls: bool,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    directions
        .into_iter()
        .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .filter(|(x, y)| x < &map[0].len() && y < &map.len())
        .filter(move |&(x, y)| map[y][x] != '#' || ignore_walls)
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
