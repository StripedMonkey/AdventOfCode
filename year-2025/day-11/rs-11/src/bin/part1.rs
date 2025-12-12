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
        "you",
        |&pos| devices.get(pos).unwrap().iter().map(|s| *s),
        |&pos| pos == "out",
    );
    println!("Number of paths: {}", paths);
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use itertools::Itertools as _;

    #[test]
    fn first_test() {
        let input = rs_2025_11::static_read("example1.txt");
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
            "you",
            |&pos| devices.get(pos).unwrap().iter().map(|s| *s),
            |&pos| pos == "out",
        );
        println!("Number of paths: {}", paths);
        assert_eq!(paths, 5);
    }
}
