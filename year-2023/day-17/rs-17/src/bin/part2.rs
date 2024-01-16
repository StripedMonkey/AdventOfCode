use rs_2023_17::*;

fn main() {
    let file = *INPUT_1;
    let map = parse_file(&file);
    let result = map.puzzle::<4, 10>();
    println!("The answer is {}", result); // 1283
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_17::static_read("example1.txt");
        let map = parse_file(&file);
        let result = map.puzzle::<4, 10>();
        assert_eq!(result, 94);
    }

    #[test]
    fn second_test() {
        let file = rs_2023_17::static_read("example2.txt");
        let map = parse_file(&file);
        let result = map.puzzle::<4, 9>();
        assert_eq!(result, 71);
    }
}
