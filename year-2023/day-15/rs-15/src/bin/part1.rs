use rs_2023_15::*;

fn main() {
    let file = *INPUT_1;
    let result = split_operations(&file)
        .map(|op| hash_str(op))
        .sum::<usize>();
    println!("The answer is {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_15::static_read("example1.txt");
        let result = split_operations(&file)
            .map(|op| hash_str(op))
            .sum::<usize>();
        assert_eq!(result, 1320);
    }
}
