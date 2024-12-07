use std::collections::HashMap;

use itertools::Itertools;
use rs_2024_06::*;


fn main() {
    let input = rs_2024_06::static_read("input2.txt");
    let map = parse(input);
    let starting_location = map
        .iter()
        .enumerate()
        .find_map(|(i, v)| {
            if let Some(j) = v.iter().position(|c| "^".contains(*c)) {
                return Some((i, j));
            }
            None
        })
        .unwrap();
    let mut unaltered_path = map.clone();
    let mut current_position = starting_location;
    let mut possible_loops = vec![];
    loop {
        let (i, j) = current_position;
        match step(&unaltered_path, (i, j)) {
            Step::End => {
                break;
            }
            Step::Move(new_pos) => {
                if new_pos != starting_location {
                    let mut test_map = map.clone();
                    test_map[new_pos.0][new_pos.1] = '#';
                    if contains_loop(test_map, starting_location) {
                        possible_loops.push(new_pos);
                    }
                }
                unaltered_path[new_pos.0][new_pos.1] = unaltered_path[i][j];
                unaltered_path[i][j] = '.';
                current_position = new_pos;
            }
            Step::Rotate(new_dir) => {
                unaltered_path[i][j] = new_dir;
            }
        }
    }
    let loops = possible_loops.iter().unique().count();
    println!("{}", loops);
}

#[derive(Debug, Clone, Copy)]
struct Directions {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Directions {
    fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    fn mark_dir(&mut self, c: char) -> bool {
        let same = match c {
            '^' => self.up,
            'v' => self.down,
            '<' => self.left,
            '>' => self.right,
            _ => panic!(),
        };
        match c {
            '^' => self.up = true,
            'v' => self.down = true,
            '<' => self.left = true,
            '>' => self.right = true,
            _ => panic!(),
        };
        same
    }
}

fn contains_loop(mut map: Vec<Vec<char>>, mut current_position: (usize, usize)) -> bool {
    let mut direction_map = HashMap::<(usize, usize), Directions>::new();
    loop {
        let (i, j) = current_position;
        match step(&map, (i, j)) {
            Step::End => {
                return false;
            }
            Step::Move(new_pos) => {
                if let Some(dir) = direction_map.get_mut(&new_pos) {
                    if dir.mark_dir(map[i][j]) {
                        return true;
                    }
                } else {
                    let mut direction = Directions::new();
                    direction.mark_dir(map[i][j]);
                    direction_map.insert(new_pos, direction);
                }
                map[new_pos.0][new_pos.1] = map[i][j];
                map[i][j] = '.';
                current_position = new_pos;
            }
            Step::Rotate(new_dir) => {
                if let Some(dir) = direction_map.get_mut(&current_position) {
                    if dir.mark_dir(new_dir) {
                        return true;
                    }
                } else {
                    let mut direction = Directions::new();
                    direction.mark_dir(new_dir);
                    direction_map.insert(current_position, direction);
                }
                map[i][j] = new_dir;
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_06::static_read("example1.txt");
        let map = parse(input);
        let starting_location = map
            .iter()
            .enumerate()
            .find_map(|(i, v)| {
                if let Some(j) = v.iter().position(|c| "^".contains(*c)) {
                    return Some((i, j));
                }
                None
            })
            .unwrap();
        let mut unaltered_path = map.clone();
        let mut current_position = starting_location;
        let mut possible_loops = vec![];
        loop {
            let (i, j) = current_position;
            match step(&unaltered_path, (i, j)) {
                Step::End => {
                    break;
                }
                Step::Move(new_pos) => {
                    if new_pos != starting_location {
                        let mut test_map = map.clone();
                        test_map[new_pos.0][new_pos.1] = '#';
                        if contains_loop(test_map.clone(), starting_location) {
                            test_map[new_pos.0][new_pos.1] = 'O';
                            println!("{:?}", possible_loops);
                            for row in test_map {
                                println!("{:?}", row);
                            }
                            possible_loops.push(new_pos);
                        }
                    }
                    unaltered_path[new_pos.0][new_pos.1] = unaltered_path[i][j];
                    unaltered_path[i][j] = '.';
                    current_position = new_pos;
                }
                Step::Rotate(new_dir) => {
                    unaltered_path[i][j] = new_dir;
                }
            }
        }
        let loops = possible_loops.iter().unique().count();
        println!("{}", loops);
        assert!(loops == 6);
    }
}

