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
                antinodes(a, b, map.bounds, 1, 2)
            })
        })
        .flatten()
        .flatten()
        .unique()
        .collect();
    let result = result.len();
    println!("{}", result);
    assert!(result == 329);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_08::static_read("example1.txt");
        let map = parse(input);
        let result: Vec<_> = map
            .towers
            .iter()
            .map(|(_, coords)| {
                coords.iter().permutations(2).map(move |x| {
                    let a = (x[0].0 as isize, x[0].1 as isize);
                    let b = (x[1].0 as isize, x[1].1 as isize);
                    antinodes(a, b, map.bounds, 1, 2)
                })
            })
            .flatten()
            .flatten()
            .unique()
            .collect();
        let result = result.len();
        println!("{}", result);
        assert!(result == 14);
    }
}
