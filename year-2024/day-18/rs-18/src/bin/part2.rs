use std::{collections::HashSet, iter};

use aoc_utils::*;

use itertools::Itertools;
use pathfinding::prelude::{astar, dijkstra};
use rs_2024_18::*;

fn main() {
    let input = rs_2024_18::static_read("input1.txt");
    let limits = (70, 70);
    let mut all_obst = parse(&input).into_iter();
    let mut obstacles = Vec::new();
    (&mut all_obst).take(1024).for_each(|x| {
        obstacles.push(x);
    });
    let mut last = (0,0);
    loop {
        if let Some(obst) = all_obst.next() {
            obstacles.push(obst);
            last = obst;
            obstacles.sort();
        } else {
            println!("No solution");
            break;
        }
        let Some(ans) = astar(
            &(0, 0),
            |p| adjacent(limits, *p, &obstacles).map(|p| (p, 1)),
            |p| distance(p, &limits),
            |p| *p == limits,
        ) else {
            break;
        };
        println!("{:?}", obstacles.len());
    }
    let ans = last;
    println!("{:?}", ans);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use pathfinding::prelude::astar;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_18::static_read("example1.txt");
        let limits = (6, 6);
        let mut all_obst = parse(&input).into_iter();
        let mut obstacles = Vec::new();
        (&mut all_obst).take(12).for_each(|x| {
            obstacles.push(x);
        });
        let mut last = (0,0);
        loop {
            if let Some(obst) = all_obst.next() {
                obstacles.push(obst);
                last = obst;
                obstacles.sort();
            } else {
                println!("No solution");
                break;
            }
            let Some(ans) = astar(
                &(0, 0),
                |p| adjacent(limits, *p, &obstacles).map(|p| (p, 1)),
                |p| distance(p, &limits),
                |p| *p == limits,
            ) else {
                break;
            };
            println!("{:?}", obstacles.len());
        }
        let ans = last;
        println!("{:?}", ans);
    }
}

fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
}

fn adjacent<'a>(
    limits: (usize, usize),
    position: (usize, usize),
    obstacles: &'a Vec<(usize, usize)>,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .map(move |(dx, dy)| (position.0 as isize + dx, position.1 as isize + dy))
        .filter(move |&(x, y)| x >= 0 && y >= 0 && x <= limits.0 as isize && y <= limits.1 as isize)
        .filter(move |&(x, y)| !obstacles.binary_search(&(x as usize, y as usize)).is_ok())
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
