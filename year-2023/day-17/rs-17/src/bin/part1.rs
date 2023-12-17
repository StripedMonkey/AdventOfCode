use rs_17::*;

fn main() {
    let file = *INPUT_1;
    let map = parse_file(&file);
    let result = map.puzzle::<0,3>();
    println!("The answer is {}", result); // 1155
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_17::static_read("example1.txt");
        let map = parse_file(&file);
        let result = map.puzzle::<0,3>();
        assert_eq!(result, 102);
    }
}
