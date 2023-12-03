use rs_03::*;

fn main() {
    let input = *INPUT_1;
    let s = Schematic::new(input);
    println!("{s}");
    let gear_ratio_sum = s
        .location_by_type('*')
        .filter_map(|(x, y)| s.gear_ratio(x, y))
        .map(|(a, b)| a * b)
        .sum::<usize>();
    println!("Gear Ratio Totals: {gear_ratio_sum}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_03::static_read("example1.txt");
        let s = Schematic::new(input);
        println!("{s}");
        let gear_ratio_sum = s
            .location_by_type('*')
            .filter_map(|(x, y)| s.gear_ratio(x, y))
            .map(|(a, b)| a * b)
            .sum::<usize>();
        println!("Gear Ratio Totals: {gear_ratio_sum}");
        assert_eq!(gear_ratio_sum, 467835);
    }
}
