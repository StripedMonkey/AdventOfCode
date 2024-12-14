use std::collections::{HashSet, VecDeque};

use rs_2024_12::*;

fn main() {
    let input = rs_2024_12::static_read("input1.txt");
    let map = parse(input);
    let regions = build_regions(&map);
    let answer: usize = regions
        .iter()
        .map(|region| {
            region.len() * {
                region
                    .iter()
                    .map(|(x, y)| edges(&map, (*x, *y)))
                    .sum::<usize>()
            }
        })
        .sum();
    println!("{}", answer);
    assert!(answer == 1930);
}

#[cfg(test)]
mod test {

    use std::collections::HashSet;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_12::static_read("example1.txt");
        let map = parse(input);
        let regions = build_regions(&map);
        let answer: usize = regions
            .iter()
            .map(|region| {
                println!("{:?}", region);

                let e = region.iter().next().unwrap();
                let edges = region
                    .iter()
                    .map(|(x, y)| edges(&map, (*x, *y)))
                    .sum::<usize>();
                println!(
                    "Region {} has {} area and {} edges",
                    map[e.1][e.0],
                    region.len(),
                    edges
                );
                region.len() * edges
            })
            .sum();
        println!("{}", answer);
        assert!(answer == 1930);
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
            println!("Building region at {:?} ({})", (i, j), map[j][i]);
            let region = build_region(&map, (i, j));
            regions.push(region);
        }
    }
    regions
}

fn build_region(
    map: &Vec<Vec<char>>,
    starting_position: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut region: HashSet<(usize, usize)> = HashSet::new();
    println!(
        "Building region {:?}",
        map[starting_position.1][starting_position.0]
    );
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

fn edges(map: &Vec<Vec<char>>, location: (usize, usize)) -> usize {
    let (start_x, start_y) = location;
    let mut count = adjacent(map, location)
        .filter(|(x, y)| map[*y][*x] != map[start_y][start_x])
        .count();
    if start_x == 0 {
        count +=1;
    }
    if start_x == map[0].len() - 1 {
        count +=1;
    }
    if start_y == 0 {
        count +=1;
    }
    if start_y == map.len() - 1 {
        count +=1;
    }
    count
}
