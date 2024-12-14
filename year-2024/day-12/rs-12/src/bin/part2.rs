use std::collections::{HashSet, VecDeque};

use itertools::Itertools as _;
use rs_2024_12::*;

fn main() {
    let input = rs_2024_12::static_read("input1.txt");
    let map = parse(input);
    let regions = build_regions(&map);
    let answer: usize = regions
        .iter()
        .map(|region| {
            let sides: usize = sides(&map, region);
            let e = region.iter().next().unwrap();
            println!(
                "Region {} has {} area and {} sides",
                map[e.1][e.0],
                region.len(),
                sides
            );
            region.len() * sides
        })
        .sum();
    println!("{}", answer);
    assert!(answer != 744445);
}

#[cfg(test)]
mod test {

    use std::collections::HashSet;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_12::static_read("example3.txt");
        let map = parse(input);
        let regions = build_regions(&map);
        let answer: usize = regions
            .iter()
            .map(|region| {
                let sides: usize = sides(&map, region);
                region.len() * sides
            })
            .sum();
        println!("{}", answer);
        assert!(answer == 80);
    }
    #[test]
    fn second_test() {
        let input = rs_2024_12::static_read("example2.txt");
        let map = parse(input);
        let regions = build_regions(&map);
        let answer: usize = regions
            .iter()
            .map(|region| {
                let sides: usize = sides(&map, region);
                region.len() * sides
            })
            .sum();
        println!("{}", answer);
        assert!(answer == 436);
    }
    #[test]
    fn third_test() {
        let input = rs_2024_12::static_read("example4.txt");
        let map = parse(input);
        let regions = build_regions(&map);
        let answer: usize = regions
            .iter()
            .map(|region| {
                let sides: usize = sides(&map, region);
                region.len() * sides
            })
            .sum();
        println!("{}", answer);
        assert!(answer == 368);
    }
    #[test]
    fn fourth_test() {
        let input = rs_2024_12::static_read("example5.txt");
        let map = parse(input);
        let regions = build_regions(&map);
        let answer: usize = regions
            .iter()
            .map(|region| {
                let sides: usize = sides(&map, region);
                let e = region.iter().next().unwrap();
                println!(
                    "Region {} has {} area and {} sides",
                    map[e.1][e.0],
                    region.len(),
                    sides
                );
                region.len() * sides
            })
            .sum();
        println!("{}", answer);
        assert!(answer == 236);
    }
}
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn build_regions(map: &Vec<Vec<char>>) -> Vec<HashSet<(usize, usize)>> {
    println!("Building regions");
    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    let height = map.len();
    let width = map[0].len();
    for i in 0..height {
        for j in 0..width {
            if regions.iter().any(|region| region.contains(&(i, j))) {
                continue;
            }
            let region = build_region(&map, (i, j));
            regions.push(region);
        }
    }
    regions
}

fn sides(map: &Vec<Vec<char>>, region: &HashSet<(usize, usize)>) -> usize {
    let mut sides: usize = 0;
    let y_max = map.len();
    // let e = region.iter().next().unwrap();
    // println!("Region {}", map[e.1][e.0]);

    sides += (-1..=y_max as isize)
        .into_iter()
        .tuple_windows()
        .map(|(y1, y2)| {
            let x_max = map[0].len();
            let line = (-1..=x_max as isize)
                .into_iter()
                .map(move |x| ((x, y1), (x, y2)));
            count_segments(region, line)
        })
        .sum::<usize>();
    // println!("Y sides {sides}");
    let x_max = map[0].len();
    sides += (-1..=x_max as isize)
        .into_iter()
        .tuple_windows()
        .map(|(x1, x2)| {
            let y_max = map.len();
            let line = (-1..=y_max as isize)
                .into_iter()
                .map(move |y| ((x1, y), (x2, y)));
            count_segments(region, line)
        })
        .sum::<usize>();
    sides
}

fn count_segments(
    region: &HashSet<(usize, usize)>,
    line: impl Iterator<Item = ((isize, isize), (isize, isize))>,
) -> usize {
    let mut segments = 0;
    let mut on_edge_a = false;
    let mut on_edge_b = false;
    for (p1, p2) in line {
        // Check if the left point is on a right facing edge
        if region.contains(&(p1.0 as usize, p1.1 as usize)) {
            // Check if we just got onto this edge
            if !region.contains(&(p2.0 as usize, p2.1 as usize)) && !on_edge_a {
                on_edge_a = true;
                segments += 1;
            }
            if region.contains(&(p2.0 as usize, p2.1 as usize)) {
                on_edge_a = false;
            }
        } else {
            on_edge_a = false;
        }
        // Check if the right point is on a left facing edge
        if region.contains(&(p2.0 as usize, p2.1 as usize)) {
            if !region.contains(&(p1.0 as usize, p1.1 as usize)) && !on_edge_b {
                on_edge_b = true;
                segments += 1;
            }
            if region.contains(&(p1.0 as usize, p1.1 as usize)) {
                on_edge_b = false;
            }
        } else {
            on_edge_b = false;
        }
    }
    // println!(
    //     "Found segments {} e1 {} e2 {}",
    //     segments, on_edge_a, on_edge_b
    // );
    segments
}

fn build_region(
    map: &Vec<Vec<char>>,
    starting_position: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut region: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back(starting_position);
    region.insert(starting_position);

    while let Some((x, y)) = queue.pop_front() {
        for (x, y) in adjacent(&map, (x, y)) {
            if region.contains(&(x, y)) {
                continue;
            }
            if map[y][x] == map[starting_position.1][starting_position.0] {
                region.insert((x, y));
                queue.push_back((x, y));
            }
        }
    }
    region
}

fn adjacent(
    map: &Vec<Vec<char>>,
    position: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let (x, y) = position;
    let max_x = map[0].len();
    let max_y = map.len();
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .filter(move |(x, y)| (0..max_y).contains(y) && (0..max_x).contains(x))
}
