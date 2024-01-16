use rs_2023_14::*;

fn main() {
    let file = *INPUT_1;
    let map = parse_file(file);
    let tilted = map.tilt_north();
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
        let tilted = map.tilt_north();
        let result: usize = tilted.load();
        println!("{result}")
    }
}
