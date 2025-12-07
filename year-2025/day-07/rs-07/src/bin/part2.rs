use std::collections::{HashMap, HashSet};

fn main() {
    let input = rs_2025_07::static_read("input1.txt");
    let mut lines = input.lines();
    let mut beam_locations = HashMap::new();
    if let Some(first_line) = lines.next() {
        for (x, c) in first_line.chars().enumerate() {
            if c == 'S' {
                beam_locations.insert(x, 1);
            }
        }
    }
    for line in lines {
        let mut new_beam_locations = HashMap::new();
        for (beam_location, possible_paths_taken) in beam_locations {
            match line.chars().nth(beam_location) {
                Some('.') => match new_beam_locations.get_mut(&beam_location) {
                    Some(existing_paths) => *existing_paths += possible_paths_taken,
                    None => {
                        new_beam_locations.insert(beam_location, possible_paths_taken);
                    }
                },
                Some('^') => {
                    for dir in [-1, 1] {
                        let new_location = (beam_location as isize + dir) as usize;
                        match new_beam_locations.get_mut(&new_location) {
                            Some(existing_paths) => *existing_paths += possible_paths_taken,
                            None => {
                                new_beam_locations.insert(new_location, possible_paths_taken);
                            }
                        };
                    }
                }
                c => panic!("Unexpected character {c:?}"),
            }
        }
        beam_locations = new_beam_locations;
    }
    println!("{:?}", beam_locations.values().sum::<usize>());
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn first_test() {
        let input = rs_2025_07::static_read("example1.txt");
        let mut lines = input.lines();
        let mut beam_locations = HashMap::new();
        if let Some(first_line) = lines.next() {
            for (x, c) in first_line.chars().enumerate() {
                if c == 'S' {
                    beam_locations.insert(x, 1);
                }
            }
        }
        for line in lines {
            let mut new_beam_locations = HashMap::new();
            for (beam_location, possible_paths_taken) in beam_locations {
                match line.chars().nth(beam_location) {
                    Some('.') => {
                        let Some(existing_paths) = new_beam_locations.get_mut(&beam_location)
                        else {
                            new_beam_locations.insert(beam_location, possible_paths_taken);
                            continue;
                        };
                        *existing_paths += possible_paths_taken;
                    }
                    Some('^') => {
                        for dir in [-1, 1] {
                            let new_location = (beam_location as isize + dir) as usize;
                            if let Some(existing_paths) = new_beam_locations.get_mut(&new_location)
                            {
                                *existing_paths += possible_paths_taken;
                            } else {
                                new_beam_locations.insert(new_location, possible_paths_taken);
                            };
                        }
                    }
                    c => panic!("Unexpected character {c:?}"),
                }
            }
            beam_locations = new_beam_locations;
        }
        println!("{:?}", beam_locations.values().sum::<usize>());
        assert_eq!(beam_locations.values().sum::<usize>(), 40);
    }
}
