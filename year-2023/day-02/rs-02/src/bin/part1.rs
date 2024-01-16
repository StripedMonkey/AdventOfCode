use rs_2023_02::*;
const MAX_RGB: Rgb = (12, 13, 14);

fn main() {
    // Find only games which can contain 12 red, 13 green, and 14 blue
    let input = *INPUT_1;
    let result = input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .filter(|game| game.possible_game(MAX_RGB))
        .map(|game| game.n)
        .sum::<usize>();
    println!("The answer is {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2023_02::static_read("example1.txt");
        let num = input
            .lines()
            .map(|line| Game::from_str(line).unwrap())
            .filter(|game| game.possible_game(MAX_RGB))
            .map(|game| game.n)
            .sum::<usize>();
        assert_eq!(num, 8);
    }
}
