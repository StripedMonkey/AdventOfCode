use std::collections::{HashSet, VecDeque};

use rs_2024_10::*;

fn main() {
    let input = rs_2024_10::static_read("input1.txt");
    let (starting_positions, map) = parse(input);
    let result: usize = starting_positions
        .iter()
        .map(|(x, y)| unique_paths(&map, (*x, *y)))
        .sum();
    println!("{}", result);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_10::static_read("example2.txt");
        let (starting_positions, map) = parse(input);
        let result: usize = starting_positions
            .iter()
            .map(|(x, y)| unique_paths(&map, (*x, *y)))
            .sum();
        println!("{}", result);
        assert!(result == 81);
    }
}

fn adjacent(
    map: &Vec<Vec<usize>>,
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let max_x = map[0].len();
    let max_y = map.len();
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        // This only works because when X < 0, `as usize` makes it > isize::MAX. I use the trick, it's dirty though.
        .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .filter(move |(x, y)| (0..max_y).contains(y) && (0..max_x).contains(x))
}

fn unique_paths(map: &Vec<Vec<usize>>, starting_position: (usize, usize)) -> usize {
    let (x, y) = starting_position;
    if map[y][x] == 9 {
        return 1;
    }
    let current = map[y][x];
    adjacent(&map, x, y)
        .filter(|(x, y)| map[*y][*x] == current + 1)
        .map(|pos| unique_paths(&map, pos))
        .sum()
}