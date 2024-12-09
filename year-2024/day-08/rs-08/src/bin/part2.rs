use std::collections::HashSet;

use itertools::Itertools;
use rs_2024_08::*;

fn antinodes(
    (ax, ay): (isize, isize),
    (bx, by): (isize, isize),
    bounds: (usize, usize),
) -> Vec<(isize, isize)> {
    let mut anti_nodes: Vec<(isize, isize)> = Vec::new();
    let (dx, dy) = (ax - bx, ay - by);
    let mut harmonic = 0;

    loop {
        let antinode_a = (ax + (dx * harmonic), ay + (dy * harmonic));
        let antinode_b = (bx - (dx * harmonic), by - (dy * harmonic));
        if (0..bounds.0 as isize).contains(&antinode_a.0)
            && (0..bounds.1 as isize).contains(&antinode_a.1)
        {
            anti_nodes.push(antinode_a);
        }
        if (0..bounds.0 as isize).contains(&antinode_b.0)
            && (0..bounds.1 as isize).contains(&antinode_b.1)
        {
            anti_nodes.push(antinode_b);
        }
        if ((0..bounds.0 as isize).contains(&antinode_a.0)
            && (0..bounds.1 as isize).contains(&antinode_a.1))
            || ((0..bounds.0 as isize).contains(&antinode_b.0)
                && (0..bounds.1 as isize).contains(&antinode_b.1))
        {
            harmonic += 1;
        } else {
            break anti_nodes;
        }
    }
}

fn main() {
    let input = rs_2024_08::static_read("input1.txt");
    let map = parse(input);
    let a = map
        .towers
        .iter()
        .map(|(_, coords)| {
            coords.iter().permutations(2).map(move |x| {
                let a = (x[0].0 as isize, x[0].1 as isize);
                let b = (x[1].0 as isize, x[1].1 as isize);
                antinodes(a, b, map.bounds)
            })
        })
        .flatten()
        .flatten()
        .collect_vec();

    let a: HashSet<_> = a.into_iter().collect();
    let result = a.len();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_08::static_read("example1.txt");
        let map = parse(input);
        let a = map
            .towers
            .iter()
            .map(|(tower, coords)| {
                coords.iter().permutations(2).map(move |x| {
                    let a = (x[0].0 as isize, x[0].1 as isize);
                    let b = (x[1].0 as isize, x[1].1 as isize);
                    antinodes(a, b, map.bounds)
                })
            })
            .flatten()
            .flatten()
            .collect_vec();

        let a: HashSet<_> = a.into_iter().collect();
        println!("{:?}", a);

        let result = a.len();
        println!("{}", result);
        assert!(result == 34);
    }

    #[test]
    fn second_test() {
        let input = rs_2024_08::static_read("example2.txt");
        let map = parse(input);
        let a = map
            .towers
            .iter()
            .map(|(tower, coords)| {
                coords.iter().permutations(2).map(move |x| {
                    let a = (x[0].0 as isize, x[0].1 as isize);
                    let b = (x[1].0 as isize, x[1].1 as isize);
                    antinodes(a, b, map.bounds)
                })
            })
            .flatten()
            .flatten()
            .collect_vec();

        let a: HashSet<_> = a
            .into_iter()
            .filter(|coord| {
                let (x, y) = *coord;
                (0..map.bounds.0 as isize).contains(&x) && (0..map.bounds.1 as isize).contains(&y)
            })
            .collect();
        println!("{:?}", a);

        let result = a.len();
        println!("{}", result);
        assert!(result == 12);
    }

    #[test]
    fn single_antinode() {
        let a = (5, 5);
        let b = (6, 6);
        let bounds = (10, 10);
        let result = antinodes(a, b, bounds);
    }
}
