use std::collections::HashSet;

use itertools::Itertools;
use rs_2024_08::*;

fn main() {
    let input = rs_2024_08::static_read("input1.txt");
    let map = parse(input);
    let result: Vec<_> = map
        .towers
        .iter()
        .map(|(_, coords)| {
            coords.iter().permutations(2).map(move |x| {
                let a = (x[0].0 as isize, x[0].1 as isize);
                let b = (x[1].0 as isize, x[1].1 as isize);
                antinodes(a, b, map.bounds, 0, isize::MAX as usize)
            })
        })
        .flatten()
        .flatten()
        .unique()
        .collect();
    let result = result.len();
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
            .map(|(_, coords)| {
                coords.iter().permutations(2).map(move |x| {
                    let a = (x[0].0 as isize, x[0].1 as isize);
                    let b = (x[1].0 as isize, x[1].1 as isize);
                    antinodes(a, b, map.bounds, 0,isize::MAX as usize)
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
            .map(|(_, coords)| {
                coords.iter().permutations(2).map(move |x| {
                    let a = (x[0].0 as isize, x[0].1 as isize);
                    let b = (x[1].0 as isize, x[1].1 as isize);
                    antinodes(a, b, map.bounds, 0,isize::MAX as usize)
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
        let result = antinodes(a, b, bounds, 0,usize::MAX);
    }
}
