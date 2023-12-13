use rs_13::*;

fn main() {
    let file = *INPUT_1;
    let (_, mirror_inputs) = parse_file(file).unwrap();
    let result = mirror_inputs
        .iter()
        // .inspect(|mirror| println!("Mirror:\n{mirror}"))
        .map(|mirror_input| mirror_input.puzzle_fuzzy())
        // .inspect(|mirror| println!("Result: {:?}", mirror))
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
        let input = rs_13::static_read("example1.txt");
        let (_, mirror_inputs) = parse_file(input).unwrap();
        let result = mirror_inputs
            .iter()
            .map(|mirror_input| mirror_input.puzzle_fuzzy())
            .fold((0, 0), |(vert, horizontal), (maybe_vert, maybe_horiz)| {
                (
                    vert + maybe_vert.unwrap_or(0),
                    horizontal + maybe_horiz.unwrap_or(0),
                )
            });
        println!("Final Result: {result:?}");
        let result = result.0 + (100 * result.1);
        assert_eq!(result, 400);
    }

    #[test]
    #[ignore = "Relies on a set of inputs and known outputs not provided"]
    fn problematic_inputs() {
        let answers = rs_13::static_read("answers1.txt");
        let answers = answers
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let input = rs_13::static_read("input1.txt");
        let (_, mirror_inputs) = parse_file(input).unwrap();
        let result = mirror_inputs
            .iter()
            .zip(answers.iter())
            // .inspect(|mirror| println!("Mirror:\n{mirror}"))
            .map(|(mirror_input, answer)| {
                let (vertical, horizontal) = mirror_input.puzzle_fuzzy();
                if let Some(result) = vertical {
                    assert_eq!(
                        result, *answer,
                        "Failing Test Case with val {result} (should be {answer}):\n{mirror_input}"
                    );
                } else if let Some(result) = horizontal {
                    assert_eq!(
                        result, *answer,
                        "Failing Test Case with val {result} (should be {answer}):\n{mirror_input}"
                    );
                }
                (vertical, horizontal)
            })
            // .inspect(|mirror| println!("Result: {:?}", mirror))
            .fold((0, 0), |(vert, horizontal), (maybe_vert, maybe_horiz)| {
                (
                    vert + maybe_vert.unwrap_or(0),
                    horizontal + maybe_horiz.unwrap_or(0),
                )
            });
        println!("Final Result: {result:?}");
        let result = result.0 + (100 * result.1);
        assert_eq!(result, 36919);
    }

    #[test]
    fn third_test() {
        let input = rs_13::static_read("example2.txt");
        let (_, mirror_inputs) = parse_file(input).unwrap();
        let result = mirror_inputs
            .iter()
            // .inspect(|mirror| println!("Mirror:\n{mirror}"))
            .map(|mirror_input| mirror_input.puzzle_fuzzy())
            // .inspect(|mirror| println!("Result: {:?}", mirror))
            .fold((0, 0), |(vert, horizontal), (maybe_vert, maybe_horiz)| {
                (
                    vert + maybe_vert.unwrap_or(0),
                    horizontal + maybe_horiz.unwrap_or(0),
                )
            });
        println!("Final Result: {result:?}");
        let result = result.0 + (100 * result.1);
        assert_eq!(result, 5);
    }

    #[test]
    fn fourth_test() {
        let input = rs_13::static_read("example3.txt");
        let (_, mirror_inputs) = parse_file(input).unwrap();
        let result = mirror_inputs
            .iter()
            // .inspect(|mirror| println!("Mirror:\n{mirror}"))
            .map(|mirror_input| mirror_input.puzzle_fuzzy())
            // .inspect(|mirror| println!("Result: {:?}", mirror))
            .fold((0, 0), |(vert, horizontal), (maybe_vert, maybe_horiz)| {
                (
                    vert + maybe_vert.unwrap_or(0),
                    horizontal + maybe_horiz.unwrap_or(0),
                )
            });
        println!("Final Result: {result:?}");
        let result = result.0 + (100 * result.1);
        assert_eq!(result, 1600);
    }
}
