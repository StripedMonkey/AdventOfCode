use rs_2025_06::*;

fn main() {
    let input = rs_2025_06::static_read("input1.txt");
    let problems = parse(&input);
    let mut total = 0;
    for problem in problems {
        match problem[problem.len() - 1].trim() {
            "*" => {
                let product: usize = values(&problem[..problem.len() - 1]).into_iter().product();
                total += product;
            }
            "+" => {
                let sum: usize = values(&problem[..problem.len() - 1]).into_iter().sum();
                total += sum;
            }
            _ => todo!(),
        }
    }
    println!("{}", total);
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    // The whitespace matters, so determine the width of each column first
    let number_widths = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.len())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    let column_widths = (0..number_widths[0].len())
        .map(|i| number_widths.iter().map(|row| row[i]).max().unwrap())
        .collect::<Vec<_>>();
    let mut problems: Vec<Vec<&str>> = Vec::new();

    for line in input.lines() {
        let mut start = 0;
        for (i, &width) in column_widths.iter().enumerate() {
            let end = start + width;
            let s = &line[start..end];
            if problems.len() <= i {
                problems.push(vec![]);
            }
            problems[i].push(s);
            // There's a space between columns, which is not accounted for by the column widths
            start = end + 1;
        }
    }
    problems
}

fn values(s: &[&str]) -> Vec<usize> {
    let mut result = Vec::new();
    // If we have 3 digit numbers,
    // 123 45 6 we can chars().rev().zip_longest()
    let mut reversed_numbers = s.iter().map(|s| s.chars().rev()).collect::<Vec<_>>();
    loop {
        let mut current_number = 0;
        for row in &mut reversed_numbers {
            if let Some(c) = row.next() {
                if c.is_whitespace() {
                    continue;
                }
                let digit = c.to_digit(10).unwrap() as usize;
                current_number = current_number * 10 + digit;
            }
        }
        if current_number == 0 {
            break;
        }
        result.push(current_number);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2025_06::static_read("example1.txt");
        let problems = parse(&input);
        let mut total = 0;
        for problem in problems {
            match problem[problem.len() - 1].trim() {
                "*" => {
                    let product: usize =
                        values(&problem[..problem.len() - 1]).into_iter().product();
                    total += product;
                }
                "+" => {
                    let sum: usize = values(&problem[..problem.len() - 1]).into_iter().sum();
                    total += sum;
                }
                _ => todo!(),
            }
        }
        assert_eq!(total, 3263827);
    }

    #[test]
    fn values_test() {
        let nums = vec!["123", " 45", "  6"];
        let result = values(&nums);
        assert_eq!(result, vec![356, 24, 1]);
        let nums = vec!["328", "64 ", "98 "];
        let result = values(&nums);
        assert_eq!(result, vec![8, 248, 369]);
    }

    #[test]
    fn parse_test() {
        let input = "123 328\n 45 64 \n  6 98 \n*   +  ";
        let problems = parse(input);
        assert_eq!(
            problems,
            vec![
                vec!["123", " 45", "  6", "*  "],
                vec!["328", "64 ", "98 ", "+  "],
            ]
        )
    }
}
