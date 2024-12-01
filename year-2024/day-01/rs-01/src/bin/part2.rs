use rs_2024_01::*;

fn main() {
    let input = rs_2024_01::static_read("input1.txt");
    let (left, right) = parse(input);

    let answer: usize = left
        .iter()
        .map(|l| {
            let right_count = right.iter().filter(|r| *r == l).count();
            right_count * l
        })
        .sum();
    println!("{}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_01::static_read("example1.txt");
        let (mut left, mut right) = parse(input);

        let answer: usize = left
            .iter()
            .map(|l| {
                let right_count = right.iter().filter(|r| *r == l).count();
                right_count * l
            })
            .sum();
        println!("{}", answer);
        assert!(answer == 31)
    }
}
