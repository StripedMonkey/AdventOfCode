use std::{collections::HashMap, iter};

use rs_2024_11::*;

fn main() {
    let input = rs_2024_11::static_read("input1.txt");
    let mut values = parse(input);
    for _ in 0..25 {
        values = step(values);
    }
    let result: usize = values.iter().map(|v| v.1).sum();
    println!("{}", result);
    assert!(result == 182081);
}

#[cfg(test)]
mod test {
    
        use super::*;
    
        #[test]
        fn first_test() {
            let input = rs_2024_11::static_read("example1.txt");
            let mut values = parse(input);
            for _ in 0..25 {
                values = step(values);
            }
            let result: usize = values.iter().map(|v| v.1).sum();
            println!("{}", result);
            assert!(result == 55312);
        }
}