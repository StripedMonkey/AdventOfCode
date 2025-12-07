use std::collections::HashSet;

fn main() {
    let input = rs_2025_07::static_read("input1.txt");
            let mut lines = input.lines();
        let mut beam_locations = HashSet::new();
        if let Some(first_line) = lines.next() {
            for (x, c) in first_line.chars().enumerate() {
                if c == 'S' {
                    beam_locations.insert(x);
                }
            }
        }
        let mut times_split = 0;
        for line in lines {
            let mut new_beam_locations = HashSet::new();
            for beam_location in beam_locations {
                match line.chars().nth(beam_location) {
                    Some('.') => {
                        new_beam_locations.insert(beam_location);
                    }
                    Some('^') => {
                        new_beam_locations.insert(beam_location - 1);
                        new_beam_locations.insert(beam_location + 1);
                        times_split += 1;
                    }
                    c => panic!("Unexpected character {c:?}"),
                }
            }
            beam_locations = new_beam_locations;
        }
        println!("{}", times_split);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn first_test() {
        let input = rs_2025_07::static_read("example1.txt");
        let mut lines = input.lines();
        let mut beam_locations = HashSet::new();
        if let Some(first_line) = lines.next() {
            for (x, c) in first_line.chars().enumerate() {
                if c == 'S' {
                    beam_locations.insert(x);
                }
            }
        }
        let mut times_split = 0;
        for line in lines {
            let mut new_beam_locations = HashSet::new();
            for beam_location in beam_locations {
                match line.chars().nth(beam_location) {
                    Some('.') => {
                        new_beam_locations.insert(beam_location);
                    }
                    Some('^') => {
                        new_beam_locations.insert(beam_location - 1);
                        new_beam_locations.insert(beam_location + 1);
                        times_split += 1;
                    }
                    c => panic!("Unexpected character {c:?}"),
                }
            }
            beam_locations = new_beam_locations;
        }
        assert_eq!(times_split, 21);
    }
}
