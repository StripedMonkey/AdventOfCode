use rs_2023_19::*;

fn main() {
    let file = *INPUT_1;
    let (_, system) = parse_file(file).unwrap();
    let result = system.possible();
    println!("The answer is {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_19::static_read("example1.txt");
        let (_, system) = parse_file(file).unwrap();
        let result = system.possible();
        assert_eq!(result, 167_409_079_868_000);
        println!("{result:#?}")
    }
}
