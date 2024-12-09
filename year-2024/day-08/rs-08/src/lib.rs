
use std::{collections::HashMap, env, path::PathBuf, str::FromStr};


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
