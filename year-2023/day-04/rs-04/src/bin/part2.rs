use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::u32,
    multi::separated_list1,
    IResult,
};
use rs_04::*;

fn main() {
    let input = *INPUT_1;
    let cards = input
        .lines()
        .map(parse_scratchcard)
        .map(|e| e.unwrap().1)
        .collect_vec();
    let mut duplicate_cards = Vec::new();
    let result = cards.iter().rev().fold(0, |acc, card| {
        let card_count = card.matches().count();
        let sum_of_previous = duplicate_cards.iter().rev().take(card_count).sum::<usize>();
        duplicate_cards.push(sum_of_previous + 1);
        acc + sum_of_previous + 1
    });
    println!("The answer is {}", result);
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
#[derive(Debug, Clone)]
struct ScratchCard {
    card: usize,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl ScratchCard {
    fn points(&self) -> usize {
        self.your_numbers
            .iter()
            .filter(|c| {
                if self.winning_numbers.contains(c) {}
                self.winning_numbers.contains(c)
            })
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
    }
    fn matches(&self) -> impl Iterator<Item = &u32> {
        self.your_numbers.iter().filter(|c| {
            // if self.winning_numbers.contains(c) {
            //     println!("Card {} matches {}", self.card, c);
            // }
            self.winning_numbers.contains(c)
        })
    }
}

fn parse_scratchcard(line: &str) -> IResult<&str, ScratchCard> {
    let (line, _) = tag("Card")(line)?;
    let (line, _) = take_while1(|c: char| c.is_whitespace())(line)?;
    let (line, card) = u32(line)?;
    let (line, _) = tag(":")(line)?;
    let (line, _) = take_while1(|c: char| c.is_whitespace())(line)?;
    let (line, winning_numbers) =
        separated_list1(take_while1(|c: char| c.is_whitespace()), u32)(line)?;
    let (line, _) = take_while1(|c: char| c.is_whitespace())(line)?;
    let (line, _) = tag("|")(line)?;
    let (line, _) = take_while1(|c: char| c.is_whitespace())(line)?;

    let (line, your_numbers) =
        separated_list1(take_while1(|c: char| c.is_whitespace()), u32)(line)?;
    Ok((
        line,
        ScratchCard {
            card: card as usize,
            winning_numbers,
            your_numbers,
        },
    ))
}

#[cfg(test)]
mod test {

    use itertools::Itertools;

    use super::*;

    #[test]
    fn second_test() {
        let input = rs_04::static_read("example2.txt");
        let cards = input
            .lines()
            .map(parse_scratchcard)
            .map(|e| e.unwrap().1)
            .collect_vec();
        let mut duplicate_cards = Vec::with_capacity(cards.len());
        let result = cards
            .iter()
            .rev()
            .map(|c| {
                let card_count = c.matches().count();
                card_count
            })
            .fold(0, |acc, card_count| {
                let sum_of_previous = duplicate_cards.iter().rev().take(card_count).sum::<usize>();
                duplicate_cards.push(sum_of_previous + 1);
                println!(
                    "matches {} previous sum: {} duplicates: {:?}",
                    card_count, sum_of_previous, duplicate_cards
                );
                acc + sum_of_previous + 1
            });
        assert_eq!(result, 30);
    }
}
