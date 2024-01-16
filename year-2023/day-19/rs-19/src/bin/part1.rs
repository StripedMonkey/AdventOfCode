use rs_2023_19::*;

fn main() {
    let file = *INPUT_1;
    let (_, system) = parse_file(file).unwrap();
    let result = system
        .get_accepted()
        .map(|part| part.values().sum::<usize>())
        .sum::<usize>();
    println!("The answer is {}", result); // 362930
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_19::static_read("example1.txt");
        let (_, system) = parse_file(file).unwrap();
        let result = system
            .get_accepted()
            .map(|part| part.values().sum::<usize>())
            .sum::<usize>();
        assert_eq!(result, 19114);
        println!("{result:#?}")
    }
}
