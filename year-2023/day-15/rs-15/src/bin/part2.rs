use rs_2023_15::*;

fn main() {
    let file = *INPUT_1;
    let mut boxes = Boxes::new();
    let iter = parse_file(&file);
    boxes.extend(iter);
    println!("The answer is {}", boxes.focusing_power());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn second_test() {
        let file = rs_2023_15::static_read("example1.txt");
        let mut boxes = Boxes::new();
        let iter = parse_file(&file);
        boxes.extend(iter);
        let answer = boxes.focusing_power();
        assert_eq!(answer, 145);
        println!("The answer is {}", answer);
    }
}
