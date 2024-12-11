use std::collections::{HashSet, VecDeque};

use rs_2024_10::*;

fn main() {
    let input = rs_2024_10::static_read("input1.txt");
    let (starting_positions, map) = parse(input);
    let result: usize = starting_positions
        .iter()
        .map(|(x, y)| reachable_peaks(&map, (*x, *y)))
        .sum();
    println!("{}", result);
    assert!(result == 587);
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
            .map(|(x, y)| reachable_peaks(&map, (*x, *y)))
            .sum();
        println!("{}", result);
        assert!(result == 36);
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
        .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .filter(move |(x, y)| (0..max_y).contains(y) && (0..max_x).contains(x))
}

fn reachable_peaks(map: &Vec<Vec<usize>>, starting_position: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    let mut peaks: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back(starting_position);
    while let Some((x, y)) = queue.pop_front() {
        if map[y][x] == 9 {
            peaks.insert((x, y));
        }
        let current = map[y][x];
        for (x, y) in adjacent(&map, x, y) {
            if map[y][x] == current + 1 {
                queue.push_back((x, y));
            }
        }
    }
    peaks.len()
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let starting_positions = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, p)| if *p == 0 { Some((x, y)) } else { None })
        })
        .flatten()
        .collect();
    (starting_positions, map)
}
