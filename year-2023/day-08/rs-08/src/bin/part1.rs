use rs_2023_08::*;

fn main() {
    let file = *INPUT_1;
    let result = parse_network(&file).unwrap().1;
    let result = result
        .walk("AAA")
        .inspect(|i| {
            println!("Curent Node: {i:?}");
        })
        .take_while(|(_, node)| *node != "ZZZ")
        .count()
        + 1;
    println!("{result:?} Steps");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_08::static_read("example1.txt");
        let result = parse_network(&file).unwrap().1;
        let result = result
            .walk("AAA")
            .inspect(|i| {
                println!("Curent Node: {i:?}");
            })
            .take_while(|(_, node)| *node != "ZZZ")
            .count();
        assert_eq!(result, 2);
        println!("{result:?}")
    }

    #[test]
    fn second_test() {
        let file = rs_2023_08::static_read("example2.txt");
        let result = parse_network(&file).unwrap().1;
        let result = result
            .walk("AAA")
            .inspect(|i| {
                println!("Curent Node: {i:?}");
            })
            .take_while(|(_, node)| *node != "ZZZ")
            .count();
        assert_eq!(result, 6);
        println!("{result:?}")
    }
}
