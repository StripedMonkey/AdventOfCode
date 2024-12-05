use rs_2024_05::*;

fn main() {
    let input = rs_2024_05::static_read("input1.txt");
    let (ordering_rules, page_updates) = parse(input);
    let answer: usize = page_updates
        .into_iter()
        .filter(|update| is_valid(update, &ordering_rules))
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{}", answer);
    assert!(answer == 6034);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_05::static_read("example1.txt");
        let (ordering_rules, page_updates) = parse(input);
        let answer: usize = page_updates
            .iter()
            .filter(|update| is_valid(update, &ordering_rules))
            .map(|update| update[update.len() / 2])
            .sum();
        println!("{}", answer);
        assert!(answer == 143)
    }
}
