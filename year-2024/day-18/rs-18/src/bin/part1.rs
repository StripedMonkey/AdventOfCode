use std::{collections::HashSet, iter};

use aoc_utils::*;

use itertools::Itertools;
use pathfinding::prelude::{astar, dijkstra};
use rs_2024_18::*;

fn main() {
    let input = rs_2024_18::static_read("input1.txt");
    let obstacles = parse(&input);
    let obstacles = obstacles.iter().take(1024).collect::<HashSet<_>>();
    let limits = (70, 70);
    let ans = astar(
        &(0, 0),
        |p| adjacent(limits, *p, &obstacles).map(|p| (p, 1)),
        |p| distance(p, &limits),
        |p| *p == limits,
    ).unwrap();
    println!("{:?}", ans.1);

}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use pathfinding::prelude::{astar, dijkstra};

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_18::static_read("example1.txt");
        let obstacles = parse(&input);
        let obstacles = obstacles.iter().take(12).collect::<HashSet<_>>();
        let limits = (6, 6);
        let ans = astar(
            &(0, 0),
            |p| adjacent(limits, *p, &obstacles).map(|p| (p, 1)),
            |p| distance(p, &limits),
            |p| *p == limits,
        ).unwrap();
        println!("{:?}", ans.1);
        assert!(ans.1 == 22);
    }
}

fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
}

fn adjacent<'a>(
    limits: (usize, usize),
    position: (usize, usize),
    obstacles: &'a HashSet<&(usize, usize)>,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .map(move |(dx, dy)| (position.0 as isize + dx, position.1 as isize + dy))
        .filter(move |&(x, y)| x >= 0 && y >= 0 && x <= limits.0 as isize && y <= limits.1 as isize)
        .filter(move |&(x, y)| !obstacles.contains(&(x as usize, y as usize)))
        .map(|(x, y)| (x as usize, y as usize))
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let ans = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec();
            (ans[0], ans[1])
        })
        .collect()
}
