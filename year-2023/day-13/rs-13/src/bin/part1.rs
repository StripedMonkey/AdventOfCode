use rs_2023_13::*;

fn main() {
    let file = *INPUT_1;
    let (_, mirror_inputs) = parse_file(file).unwrap();
    let result = mirror_inputs
        .iter()
        .inspect(|mirror| println!("Mirror:\n{mirror}"))
        .map(|mirror_input| mirror_input.puzzle())
        .inspect(|mirror| println!("Result: {:?}", mirror))
        .fold((0, 0), |(vert, horizontal), (maybe_vert, maybe_horiz)| {
            (
                vert + maybe_vert.unwrap_or(0),
                horizontal + maybe_horiz.unwrap_or(0),
            )
        });
    println!("Final Result: {result:?}");
    let result = result.0 + (100 * result.1);
    println!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2023_13::static_read("example1.txt");
        let (_, mirror_inputs) = parse_file(input).unwrap();
        let result = mirror_inputs
            .iter()
            .map(|mirror_input| mirror_input.puzzle())
            .fold((0, 0), |(vert, horizontal), (maybe_vert, maybe_horiz)| {
                (
                    vert + maybe_vert.unwrap_or(0),
                    horizontal + maybe_horiz.unwrap_or(0),
                )
            });
        println!("Final Result: {result:?}");
        let result = result.0 + (100 * result.1);
        assert_eq!(result, 405);
    }
}
