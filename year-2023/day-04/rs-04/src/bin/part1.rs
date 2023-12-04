use itertools::Itertools;

use rs_04::*;

fn main() {
    let input = *INPUT_1;
    let result = input
        .lines()
        .map(parse_scratchcard)
        .map_ok(|e| {
            let card = e.1;
            let points = card.points();
            points
        })
        .fold_ok(0, |acc, c| acc + c)
        .unwrap();
    println!("The answer is {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_04::static_read("example1.txt");
        let result = input
            .lines()
            .map(parse_scratchcard)
            .map_ok(|e| {
                let card = e.1;
                let points = card.points();
                points
            })
            .fold_ok(0, |acc, c| acc + c)
            .unwrap();
        assert_eq!(result, 13);
    }
}
