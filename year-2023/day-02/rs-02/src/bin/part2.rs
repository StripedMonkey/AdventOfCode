use rs_2023_02::*;

fn main() {
    // Find only games which can contain 12 red, 13 green, and 14 blue
    let input = *INPUT_1;
    let result: u64 = input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .map(|game| game.lower_rgb_power())
        .sum();
    println!("The answer is {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input: &str = rs_2023_02::static_read("example1.txt");
        let num: u64 = input
            .lines()
            .map(|line| Game::from_str(line).unwrap())
            .map(|game| game.lower_rgb_power())
            .sum();
        assert_eq!(num, 2286)
    }
}
