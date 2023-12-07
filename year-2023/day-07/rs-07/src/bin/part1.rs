use std::collections::HashMap;

use itertools::Itertools;
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

fn sort_card_key(c: &char) -> u32 {
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

impl PartialOrd for CardHand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_type = self.get_type();
        let other_type = other.get_type();
        if self_type != other_type {
            return Some(self_type.cmp(&other_type))
        }
        // Priority 
        // let self_hand = self.hand.chars().sorted_by_cached_key(sort_card_key).rev().collect::<String>();
        // let other_hand = other.hand.chars().sorted_by_cached_key(sort_card_key).rev().collect::<String>();
        // println!("Self hand: {self_hand}");
        // println!("Other hand: {other_hand}");
        if let Some(x) = self.hand.chars()
            .zip(other.hand.chars())
            .find_or_first(|(a, b)| a != b)
        {
            let self_value = sort_card_key(&x.0);
            let other_value = sort_card_key(&x.1);
            return Some(other_value.cmp(&self_value));
        }
        None
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
        if card_counts.len() == 2 {
            if card_counts.iter().any(|(_, &count)| count == 4) {
                return HandType::FourOFAKind;
            }
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
fn parse_line(line: &str) -> IResult<&str, CardHand> {
    let (line, hand) = alphanumeric1(line)?;
    let (line, _) = nom::character::complete::space1(line)?;
    let (line, bid) =
        nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse::<u64>())(
            line,
        )?;
    Ok((line, CardHand { hand: hand, bid }))
}

fn main() {
    let file = *INPUT_1;
    let mut result = file
        .lines()
        .map(|line| parse_line(line).unwrap().1)
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
    println!("{result:?}");}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first_test() {
        let file = rs_07::static_read("example1.txt");
        let mut result = file
            .lines()
            .map(|line| parse_line(line).unwrap().1)
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
