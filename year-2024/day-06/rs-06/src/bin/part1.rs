use itertools::Itertools;
use rs_2024_06::*;

fn main() {
    let input = rs_2024_06::static_read("input1.txt");
    let mut map = parse(input);
    let starting_location = map.iter().enumerate().find_map(|(i, v)| {
        if let Some(j) = v.iter().position(|c| "^".contains(*c)) {
            return Some((i, j));
        }
        None
    });

    let directions: Vec<(char, (isize, isize))> =
        vec![('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))];
    let mut current_position = starting_location;
    loop {
        let Some((i, j)) = current_position else {
            break;
        };
        match step(&map, (i, j)) {
            Step::End => {
                map[i][j] = 'X';
                break;
            }
            Step::Move(new_pos) => {
                map[new_pos.0][new_pos.1] = map[i][j];
                map[i][j] = 'X';
                current_position = Some(new_pos);
            }
            Step::Rotate(new_dir) => {
                map[i][j] = new_dir;
            }
        }
    }
    let locations: usize = map
        .iter()
        .map(|v| v.iter().filter(|c| **c == 'X').count())
        .sum();
    println!("{}", locations);
    assert!(locations == 4711);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_06::static_read("example1.txt");
        let mut map = parse(input);
        let starting_location = map.iter().enumerate().find_map(|(i, v)| {
            if let Some(j) = v.iter().position(|c| "^".contains(*c)) {
                return Some((i, j));
            }
            None
        });

        let directions: Vec<(char, (isize, isize))> =
            vec![('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))];
        let mut current_position = starting_location;
        loop {
            let Some((i, j)) = current_position else {
                break;
            };
            match step(&map, (i, j)) {
                Step::End => {
                    map[i][j] = 'X';
                    break;
                }
                Step::Move(new_pos) => {
                    map[new_pos.0][new_pos.1] = map[i][j];
                    map[i][j] = 'X';
                    current_position = Some(new_pos);
                }
                Step::Rotate(new_dir) => {
                    map[i][j] = new_dir;
                }
            }
        }
        let locations: usize = map
            .iter()
            .map(|v| v.iter().filter(|c| **c == 'X').count())
            .sum();
        println!("{}", locations);
        assert!(locations == 41);
    }
}

