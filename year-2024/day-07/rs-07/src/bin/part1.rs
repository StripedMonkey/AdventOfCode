use rs_2024_07::*;

fn main() {
    let input = rs_2024_07::static_read("input1.txt");
    let data = parse(input);
    let operations = [usize::wrapping_add, usize::wrapping_mul];
    let result: usize = data
        .iter()
        .filter(|(a, b)| has_valid_perm(*a, b, &operations))
        .map(|(v, _)| v)
        .sum();
    println!("{}", result);
    assert!(result == 303766880536);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_07::static_read("example1.txt");
        let data = parse(input);
        let operations = [usize::wrapping_add, usize::wrapping_mul];

        let result: usize = data
            .iter()
            .filter(|(a, b)| has_valid_perm(*a, b, &operations))
            .map(|(v, _)| v)
            .sum();
        println!("{}", result);
        assert!(result == 3749);
    }
}
