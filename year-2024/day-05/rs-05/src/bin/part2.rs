use aoc_utils::*;
use itertools::Itertools as _;
use rs_2024_05::*;

fn main() {
    let input = rs_2024_05::static_read("input1.txt");
    let (ordering_rules, mut page_updates) = parse(input);
    // println!("{:?}", ordering_rules);
    // println!("{:?}", page_updates);
    let answer: usize = page_updates
    .into_iter()
    .filter_map(|update| {
        for rule in ordering_rules.iter() {
            let first = update.iter().find_position(|e| **e == rule.0);
            let second = update.iter().find_position(|e| **e == rule.1);

            if let (Some((first, _)), Some((second, _))) = (first, second) {
                if first > second {
                    return Some(update);
                }
            }
        }
        return None;
    })
    .map(|mut update| {
        update.sort_by(compare(&ordering_rules));
        update
    })
    .map(|update| update[update.len() / 2])
    .sum();

    // println!("{:?}", page_updates);

    println!("{}", answer);
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
        let answer: usize = page_updates
            .into_iter()
            .filter_map(|update| {
                for rule in ordering_rules.iter() {
                    let first = update.iter().find_position(|e| **e == rule.0);
                    let second = update.iter().find_position(|e| **e == rule.1);

                    if let (Some((first, _)), Some((second, _))) = (first, second) {
                        if first > second {
                            return Some(update);
                        }
                    }
                }
                return None;
            })
            .map(|mut update| {
                update.sort_by(compare(&ordering_rules));
                update
            })
            .map(|update| update[update.len() / 2])
            .sum();

        println!("{}", answer);
        assert!(answer == 123)
    }
}
