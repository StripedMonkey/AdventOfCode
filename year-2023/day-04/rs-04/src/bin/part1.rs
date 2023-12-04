use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, u32},
    multi::separated_list1,
    IResult,
};
use rs_04::*;

fn main() {
    let input = *INPUT_1;
    let result = input
        .lines()
        .map(parse_scratchcard)
        .map_ok(|e| {
            let card = e.1;
            let points = card.points();
            println!("Card {card:?} Points: {points}");
            points
        })
        .fold(0, |acc, c| acc + c.unwrap());

    println!("The answer is {}", result);
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
#[derive(Debug)]
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
            .fold(0, |acc, c| if acc == 0 { 1 } else { acc * 2 })
    }
}

fn parse_scratchcard(line: &str) -> IResult<&str, ScratchCard> {
    let (line, _) = tag("Card")(line)?;
    let (line, _) = take_while1(|c: char| c.is_whitespace())(line)?; // TODO: This is a hack, but it works for now. (I think.
    let (line, card) = u32(line)?;
    let (line, _) = tag(":")(line)?;
    let (line, _) = take_while1(|c: char| c.is_whitespace())(line)?; // TODO: This is a hack, but it works for now. (I think.
    let (line, winning_numbers) =
        separated_list1(take_while1(|c: char| c.is_whitespace()), u32)(line)?;
    let (line, _) = take_while1(|c: char| c.is_whitespace())(line)?; // TODO: This is a hack, but it works for now. (I think.
    let (line, _) = tag("|")(line)?;
    let (line, _) = take_while1(|c: char| c.is_whitespace())(line)?; // TODO: This is a hack, but it works for now. (I think.

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
    fn first_test() {
        let input = rs_04::static_read("example1.txt");
        let result = input
            .lines()
            .map(parse_scratchcard)
            .map(|e| {
                let card = e.unwrap().1;
                let points = card.points();
                println!("Card {card:?} Points: {points}");
                points
            })
            .fold(0, |acc, c| acc + c);
        assert_eq!(result, 13);
    }
}
