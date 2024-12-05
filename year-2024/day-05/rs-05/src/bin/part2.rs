use aoc_utils::*;
use rs_2024_05::*;

fn main() {
    let input = rs_2024_05::static_read("input1.txt");
    let (ordering_rules, page_updates) = parse(input);
    // println!("{:?}", ordering_rules);
    // println!("{:?}", page_updates);
    let answer: usize = page_updates
        .into_iter()
        .filter(|update| !is_valid(update, &ordering_rules))
        .map(|mut update| {
            update.sort_by(compare(&ordering_rules));
            update
        })
        .map(|update| update[update.len() / 2])
        .sum();
    // println!("{:?}", page_updates);

    println!("{}", answer);
    // assert!(answer == 6305);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_05::static_read("example1.txt");
        let (ordering_rules, page_updates) = parse(input);
        println!("{:?}", ordering_rules);
        println!("{:?}", page_updates);
        let answer: usize = page_updates
            .into_iter()
            .filter(|update| !is_valid(update, &ordering_rules))
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
