use std::{collections::HashMap, env, path::PathBuf, str::FromStr};

use itertools::Itertools as _;

// Read a file path relative to the parent of the current day's project directory.
// Each day contains the inputs and examples for that day in the the parent, so I can
// (in theory/later) share the inputs between multiple languages. Pretend like I'll actually do that.
pub fn static_read(file_path: &str) -> &'static str {
    let mut cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).expect("Infallible action failed!");
    cwd.pop();
    let file_path = cwd.join(file_path);
    println!("{:?}", file_path);
    let file = std::fs::read_to_string(file_path).expect("Failed to open file!");
    Box::leak(file.into_boxed_str())
}

pub struct Map {
    pub bounds: (usize, usize),
    pub towers: Vec<(char, Vec<(usize, usize)>)>,
}

pub fn antinodes(
    (ax, ay): (isize, isize),
    (bx, by): (isize, isize),
    bounds: (usize, usize),
    start_harmonic: usize,
    end_harmonic: usize,
) -> impl Iterator<Item = (isize, isize)> {
    let (dx, dy) = (ax - bx, ay - by);
    let pos_harmonics = (start_harmonic as isize..end_harmonic as isize)
        .map(move |harmonic| (dx * harmonic, dy * harmonic))
        .map(move |(hx, hy)| (ax + hx, ay + hy))
        .take_while(move |(x, y)| {
            (0..bounds.0 as isize).contains(x) && (0..bounds.1 as isize).contains(y)
        });
    let neg_harmonics = (start_harmonic as isize..end_harmonic as isize)
        .map(move |harmonic| (dx * harmonic, dy * harmonic))
        .map(move |(hx, hy)| (bx - hx, by - hy))
        .take_while(move |(x, y)| {
            (0..bounds.0 as isize).contains(x) && (0..bounds.1 as isize).contains(y)
        });
    pos_harmonics.chain(neg_harmonics)
}

pub fn parse(input: &str) -> Map {
    let x_len = input.lines().next().unwrap().len();
    let y_len = input.lines().count();
    let towers: Vec<(char, (usize, usize))> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| {
                if c == '.' {
                    return None;
                }
                Some((c, (i, j)))
            })
        })
        .flatten()
        .collect();
    let mut tower_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    towers.into_iter().for_each(|(c, (i, j))| {
        if let Some(v) = tower_map.get_mut(&c) {
            v.push((i, j));
        } else {
            tower_map.insert(c, vec![(i, j)]);
        }
    });
    Map {
        bounds: (x_len, y_len),
        towers: tower_map.into_iter().collect(),
    }
}
