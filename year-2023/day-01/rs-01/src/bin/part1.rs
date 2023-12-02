use rs_01::*;

// Brute force the first and last digits, since it's easy enough
fn proces_line(line: &str) -> u32 {
    let first_digit = line.chars().find(|c| c.is_numeric());
    let last_digit = line.chars().rev().find(|c| c.is_numeric());
    match (first_digit, last_digit) {
        (Some(first), Some(last)) => {
            let first = first.to_digit(10).unwrap() * 10;
            let last = last.to_digit(10).unwrap();
            first + last
        }
        _ => panic!("A line did not contain any digits"),
    }
}

fn main() {
    println!("Running Part 1:");
    let input = *INPUT_1;
    assert_eq!(input.lines().map(proces_line).sum::<u32>(), 54927);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = *INPUT_1;
        assert_eq!(input.lines().map(proces_line).sum::<u32>(), 54927);
    }
}
