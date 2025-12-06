
fn main() {
    let input = rs_2025_06::static_read("input1.txt");
    let mut problems = Vec::new();
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    first_line.split_whitespace().for_each(|p| {
        problems.push(vec![p]);
    });
    lines.for_each(|line| {
        line.split_whitespace()
            .enumerate()
            .for_each(|(i, s)| problems[i].push(s))
    });
    let mut total = 0;
    for problem in problems {
        match problem[problem.len() - 1] {
            "*" => {
                let sum: usize = problem[..problem.len() - 1]
                    .iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .product();
                total += sum;
            }
            "+" => {
                let sum: usize = problem[..problem.len() - 1]
                    .iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .sum();
                total += sum;
            }
            _ => todo!(),
        }
    }
    println!("{}", total);
}

#[cfg(test)]
mod test {

    #[test]
    fn first_test() {
        let input = rs_2025_06::static_read("example1.txt");
        let mut problems = Vec::new();
        let mut lines = input.lines();
        let first_line = lines.next().unwrap();
        first_line.split_whitespace().for_each(|p| {
            problems.push(vec![p]);
        });
        lines.for_each(|line| {
            line.split_whitespace()
                .enumerate()
                .for_each(|(i, s)| problems[i].push(s))
        });
        let mut total = 0;
        for problem in problems {
            match problem[problem.len() - 1] {
                "*" => {
                    let sum: usize = problem[..problem.len() - 1]
                        .iter()
                        .map(|s| s.parse::<usize>().unwrap())
                        .product();
                    total += sum;
                }
                "+" => {
                    let sum: usize = problem[..problem.len() - 1]
                        .iter()
                        .map(|s| s.parse::<usize>().unwrap())
                        .sum();
                    total += sum;
                }
                _ => todo!(),
            }
        }
        assert_eq!(total, 4277556);
    }
}
