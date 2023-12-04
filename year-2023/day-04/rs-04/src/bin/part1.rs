use rs_04::*;

fn main() {
    let input = *INPUT_1;
    let result = input
        .lines()
        .map(parse_scratchcard)
        .filter_map(|e| Some(e.ok()?.1.points()))
        .sum::<usize>();
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
            .filter_map(|e| Some(e.ok()?.1.points()))
            .sum::<usize>();
        assert_eq!(result, 13);
    }
}
