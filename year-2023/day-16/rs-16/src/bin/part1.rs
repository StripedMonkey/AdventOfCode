use rs_2023_16::*;

fn main() {
    let file = *INPUT_1;
    let map = parse_file(&file);
    let result = map.traverse((Direction::Right, (0, 0)));
    println!("The answer is {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_16::static_read("example1.txt");
        let map = parse_file(&file);
        let result = map.traverse((Direction::Right, (0, 0)));
        println!("Result: {}", result);
        assert_eq!(result, 46);
    }
}
