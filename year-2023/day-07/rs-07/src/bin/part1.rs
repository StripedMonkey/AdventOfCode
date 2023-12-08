use std::{cmp::Ordering, collections::HashMap};

use nom::{character::complete::alphanumeric1, IResult};
use rs_07::*;

#[derive(Debug, Eq, PartialEq)]
struct CardHand<'a> {
    hand: &'a str,
    bid: u64,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOFAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn card_value(c: &char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        c => c.to_digit(10).unwrap(),
    }
}

impl Ord for CardHand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();
        if self_type != other_type {
            return self_type.cmp(&other_type);
        }
        let self_chars = self.hand.chars();
        let other_chars = other.hand.chars();
        if let Some(x) = self_chars.zip(other_chars).find(|(a, b)| a != b) {
            let self_value = card_value(&x.0);
            let other_value = card_value(&x.1);
            return other_value.cmp(&self_value);
        }
        Ordering::Equal
    }
}

impl PartialOrd for CardHand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CardHand<'_> {
    fn get_type(&self) -> HandType {
        let binding = self.hand.chars().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });
        let card_counts = binding.iter().collect::<Vec<_>>();
        if card_counts.len() == 1 {
            return HandType::FiveOfAKind;
        }
        if card_counts.len() == 2 && card_counts.iter().any(|(_, &count)| count == 4) {
            return HandType::FourOFAKind;
        }
        if card_counts.iter().any(|(_, &count)| count == 3) {
            if card_counts.iter().any(|(_, &count)| count == 2) {
                return HandType::FullHouse;
            }
            return HandType::ThreeOfAKind;
        }
        if card_counts.len() == 3 {
            return HandType::TwoPair;
        }
        if card_counts.len() == 4 {
            return HandType::OnePair;
        }
        HandType::HighCard
    }
}

// 32T3K 765
fn parse_hand(line: &str) -> IResult<&str, CardHand> {
    let (line, hand) = alphanumeric1(line)?;
    let (line, _) = nom::character::complete::space1(line)?;
    let (line, bid) =
        nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse::<u64>())(
            line,
        )?;
    Ok((line, CardHand { hand, bid }))
}

fn main() {
    let file = *INPUT_1;
    let mut result = file
        .lines()
        .map(|line| parse_hand(line).unwrap().1)
        .collect::<Vec<_>>();
    result.sort();
    result.reverse();
    let result = result
        .iter()
        .enumerate()
        .inspect(|(i, card)| {
            println!("Hand {card:?}");
            println!("Rank {} type {:?}", i + 1, card.get_type());
        })
        .map(|(i, x)| x.bid * (i as u64 + 1))
        .sum::<u64>();
    println!("{result:?}");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first_test() {
        let file = rs_07::static_read("example1.txt");
        let mut result = file
            .lines()
            .map(|line| parse_hand(line).unwrap().1)
            .collect::<Vec<_>>();
        result.sort();
        result.reverse();
        let result = result
            .iter()
            .enumerate()
            .inspect(|(i, card)| {
                println!("Hand {card:?}");
                println!("Rank {} type {:?}", i + 1, card.get_type());
            })
            .map(|(i, x)| x.bid * (i as u64 + 1))
            .sum::<u64>();
        println!("{result:?}");
    }
}
