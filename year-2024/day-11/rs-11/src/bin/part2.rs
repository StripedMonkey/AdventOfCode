use std::collections::{HashMap, HashSet, VecDeque};

use rs_2024_11::*;

fn main() {
    let input = rs_2024_11::static_read("input1.txt");
    let mut values = parse(input);
    for _ in 0..75 {
        values = step(values);
    }
    let result: usize = values.iter().map(|v| v.1).sum();
    println!("{}", result);
    assert!(result == 216318908621637);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_11::static_read("example2.txt");
        // assert!(result == 81);
    }
}
