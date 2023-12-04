use itertools::Itertools;
use rs_04::*;

fn main() {
    let input = *INPUT_1;
    let cards = input
        .lines()
        .map(parse_scratchcard)
        .map(|e| e.unwrap().1)
        .collect_vec();
    let result = CardStack::new(cards).duplicate();
    // let cards = input
    //     .lines()
    //     .map(parse_scratchcard)
    //     .map(|e| e.unwrap().1)
    //     .collect_vec();
    // let mut duplicate_cards = Vec::with_capacity(cards.len());
    // let result = cards.iter().rev().fold(0, |acc, card| {
    //     let card_count = card.matches().count();
    //     let new_duplicates = duplicate_cards.iter().rev().take(card_count).sum::<usize>();
    //     duplicate_cards.push(new_duplicates + 1);
    //     acc + new_duplicates + 1
    // });
    println!("The answer is {}", result);
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use rs_04::{parse_scratchcard, CardStack};

    #[test]
    fn second_test() {
        let input = rs_04::static_read("example2.txt");
        let cards = input
            .lines()
            .map(parse_scratchcard)
            .map(|e| e.unwrap().1)
            .collect_vec();
        let result = CardStack::new(cards).duplicate();
        assert_eq!(result, 30);
    }
}
