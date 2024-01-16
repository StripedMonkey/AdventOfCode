use rs_2023_09::*;

fn main() {
    let file = *INPUT_1;
    let result = file
        .lines()
        .map(|line| parse_history(line).unwrap().1)
        .map(|history| history.next())
        .sum::<i64>();
    println!("The answer is {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_09::static_read("example1.txt");
        let result = file
            .lines()
            .map(|line| parse_history(line).unwrap().1)
            .map(|history| history.next())
            .sum::<i64>();
        println!("The answer is {}", result);
    }
}
