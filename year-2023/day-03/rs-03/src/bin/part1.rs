use rs_2023_03::*;

fn main() {
    let input = *INPUT_1;
    let s = Schematic::new(input);
    let label_totals = s
        .parts_locations()
        .map(|(x, y)| s.part_label_sum(x, y))
        .sum::<usize>();
    println!("Label Totals: {label_totals}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2023_03::static_read("example1.txt");
        let s = Schematic::new(input);
        let mut label_totals = 0;
        s.parts_locations().for_each(|(x, y)| {
            let n = s.part_label_sum(x, y);
            label_totals += n;
        });
        assert_eq!(label_totals, 4361)
    }
}
