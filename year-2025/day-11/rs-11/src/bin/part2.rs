use pathfinding::prelude::*;
use std::collections::HashMap;

fn count_paths_iteratively<T>(
    start: T,
    mut successors: impl FnMut(&T) -> Vec<T>,
    mut success: impl FnMut(&T) -> bool,
) -> usize
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut cache = HashMap::new();
    let mut stack = vec![start.clone()];

    'pop: while let Some(current) = stack.pop() {
        if let Some(_) = cache.get(&current) {
            continue 'pop;
        }

        if success(&current) {
            cache.insert(current, 1);
            continue 'pop;
        }

        let mut current_count = 0;
        for next in successors(&current) {
            if let Some(&n) = cache.get(&next) {
                current_count += n;
            } else {
                stack.push(current);
                stack.push(next);
                continue 'pop;
            }
        }

        cache.insert(current, current_count);
    }

    *cache.get(&start).unwrap_or(&0)
}

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
    let paths = count_paths(
        ("svr", false, false),
        |&(pos, visited_dac, visited_fft)| {
            devices
                .get(pos)
                .map(|s| {
                    s.iter()
                        .map(|&s| match (s, visited_dac, visited_fft) {
                            ("dac", false, _) => ("dac", true, visited_fft),
                            ("fft", _, false) => ("fft", visited_dac, true),
                            (other, a, b) => (other, a, b),
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

    use pathfinding::prelude::*;

    use crate::count_paths_iteratively;

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
        let paths = count_paths(
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

    #[test]
    fn test_count_paths_iteratively() {
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
        let paths = count_paths_iteratively(
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

    #[test]
    fn test_count_paths_iteratively_ans() {
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
        let paths = count_paths_iteratively(
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
        assert_eq!(paths, 367579641755680);
    }
}
