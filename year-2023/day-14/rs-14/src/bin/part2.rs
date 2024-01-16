use rs_2023_14::*;

fn main() {
    let file = *INPUT_1;
    let map = parse_file(file);
    let tilted = map.tilt_cycles(1000000000);
    // println!("{tilted}");
    let result: usize = tilted.load();
    println!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_14::static_read("example1.txt");
        let map = parse_file(file);
        let tilted = map.tilt_cycles(1000000000);
        let result: usize = tilted.load();
        assert_eq!(result, 64);
        println!("{result}")
    }
}
