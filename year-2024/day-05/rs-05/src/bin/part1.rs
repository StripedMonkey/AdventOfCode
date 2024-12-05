use aoc_utils::*;
use itertools::Itertools as _;
use rs_2024_05::*;

fn main() {
    let input = rs_2024_05::static_read("input1.txt");
    let (ordering_rules, page_updates) = parse(input);
    println!("{:?}", ordering_rules);
    println!("{:?}", page_updates);
    let answer: usize = page_updates.into_iter().filter_map(|mut update| {
        for rule in ordering_rules.iter() {
            let first = update.iter().find_position(|e| **e == rule.0);
            let second = update.iter().find_position(|e| **e == rule.1);

            if let (Some((first,_)), Some((second,_))) = (first, second) {
                if first > second {
                    return None;
                }
            }
        }
        Some(update[update.len() / 2])
    }).sum();

    println!("{}", answer);

}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let mut ordering_rules = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let Some((left, right)) = line.split_once("|") else {
            panic!("Invalid");
        };
        let left = left.parse::<usize>().unwrap();
        let right = right.parse::<usize>().unwrap();
        ordering_rules.push((left, right));
    }
    let page_updates: Vec<Vec<usize>> = lines
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    (ordering_rules, page_updates)
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_05::static_read("example1.txt");
        let (ordering_rules, page_updates) = parse(input);
        println!("{:?}", ordering_rules);
        println!("{:?}", page_updates);
        let answer: usize = page_updates.into_iter().filter_map(|mut update| {
            for rule in ordering_rules.iter() {
                let first = update.iter().find_position(|e| **e == rule.0);
                let second = update.iter().find_position(|e| **e == rule.1);

                if let (Some((first,_)), Some((second,_))) = (first, second) {
                    if first > second {
                        return None;
                    }
                }
            }
            Some(update[update.len() / 2])
        }).sum();

        println!("{}", answer);
        assert!(answer == 143)
    }
}
