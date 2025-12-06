
fn main() {
    let input = rs_2025_05::static_read("input1.txt");
    let mut lines = input.lines();
    let mut fresh_ingredient_ranges = (&mut lines)
        .take_while(|c| !c.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();
    fresh_ingredient_ranges.sort_by_key(|r| (*r.start(), *r.end()));
    let ingredients = lines
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let fresh_ingredients = ingredients
        .iter()
        .filter(|ingredient| {
            fresh_ingredient_ranges
                .iter()
                .any(|range| range.contains(ingredient))
        })
        .collect::<Vec<_>>();
    println!("{}", fresh_ingredients.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2025_05::static_read("example1.txt");
        let mut lines = input.lines();
        let mut fresh_ingredient_ranges = (&mut lines)
            .take_while(|c| !c.is_empty())
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                let start = start.parse::<usize>().unwrap();
                let end = end.parse::<usize>().unwrap();
                start..=end
            })
            .collect::<Vec<_>>();
        fresh_ingredient_ranges.sort_by_key(|r| (*r.start(), *r.end()));
        let ingredients = lines
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let fresh_ingredients = ingredients
            .iter()
            .filter(|ingredient| {
                fresh_ingredient_ranges
                    .iter()
                    .any(|range| range.contains(ingredient))
            })
            .collect::<Vec<_>>();
        println!("{}", fresh_ingredients.len());
    }
}
