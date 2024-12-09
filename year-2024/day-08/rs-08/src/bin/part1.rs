use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rs_2024_08::*;

fn main() {
    let input = rs_2024_08::static_read("input1.txt");
    let map = parse(input);
    let (mut a, b): (Vec<_>, Vec<_>) = map
        .towers
        .iter()
        .map(|(_, coords)| {
            coords.iter().permutations(2).map(move |x| {
                let (ax, ay) = (x[0].0 as isize, x[0].1 as isize);
                let (bx, by) = (x[1].0 as isize, x[1].1 as isize);
                let (dx, dy) = (ax - bx, ay - by);
                let antinode_a = (ax + dx, ay + dy);
                let antinode_b = (bx - dx, by - dy);
                (antinode_a, antinode_b)
            })
        })
        .flatten()
        .unzip();
    a.extend(b);

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
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_08::static_read("example1.txt");
        let map = parse(input);
        let (mut a, b): (Vec<_>, Vec<_>) = map
            .towers
            .iter()
            .map(|(_, coords)| {
                coords.iter().permutations(2).map(move |x| {
                    let (ax, ay) = (x[0].0 as isize, x[0].1 as isize);
                    let (bx, by) = (x[1].0 as isize, x[1].1 as isize);
                    let (dx, dy) = (ax - bx, ay - by);
                    let antinode_a = (ax + dx, ay + dy);
                    let antinode_b = (bx - dx, by - dy);
                    (antinode_a, antinode_b)
                })
            })
            .flatten()
            .unzip();
        a.extend(b);

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
        assert!(result == 14);
    }
}