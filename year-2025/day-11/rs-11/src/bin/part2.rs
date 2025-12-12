use pathfinding::prelude::*;
use std::collections::HashMap;

fn main() {
    let input = rs_2025_11::static_read("input1.txt");
        let devices = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split(":");
                let name = parts.next().unwrap();
                let outputs = parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .collect::<Vec<&str>>();
                (name, outputs)
            })
            .collect::<HashMap<&str, Vec<&str>>>();
        let paths = pathfinding::directed::count_paths::count_paths(
            ("svr", false, false),
            |(pos, visited_dac, visited_fft)| {
                devices
                    .get(pos)
                    .map(|s| {
                        s.iter()
                            .map(|s| match (*s, visited_dac, visited_fft) {
                                ("dac", false, _) => ("dac", true, *visited_fft),
                                ("fft", _, false) => ("fft", *visited_dac, true),
                                (other, a, b) => (other, *a, *b),
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_else(|| Vec::new())
            },
            |&pos| pos == ("out", true, true),
        );
        println!("Number of paths: {}", paths);
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use itertools::Itertools as _;

    #[test]
    fn first_test() {
        let input = rs_2025_11::static_read("example2.txt");
        let devices = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split(":");
                let name = parts.next().unwrap();
                let outputs = parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .collect::<Vec<&str>>();
                (name, outputs)
            })
            .collect::<HashMap<&str, Vec<&str>>>();
        let paths = pathfinding::directed::count_paths::count_paths(
            ("svr", false, false),
            |(pos, visited_dac, visited_fft)| {
                devices
                    .get(pos)
                    .map(|s| {
                        s.iter()
                            .map(|s| match (*s, visited_dac, visited_fft) {
                                ("dac", false, _) => ("dac", true, *visited_fft),
                                ("fft", _, false) => ("fft", *visited_dac, true),
                                (other, a, b) => (other, *a, *b),
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_else(|| Vec::new())
            },
            |&pos| pos == ("out", true, true),
        );
        println!("Number of paths: {}", paths);
        assert_eq!(paths, 2);
    }
}
