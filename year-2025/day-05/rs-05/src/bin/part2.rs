use std::ops::RangeInclusive;


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
    fresh_ingredient_ranges = fresh_ingredient_ranges.into_iter().fold(
        Vec::<RangeInclusive<usize>>::new(),
        |mut acc, range| {
            if let Some(last) = acc.last_mut() {
                if *last.end() >= *range.start() - 1 {
                    *last = *last.start()..=*last.end().max(range.end());
                    return acc;
                }
            }
            acc.push(range);
            acc
        },
    );
    let ingredients = fresh_ingredient_ranges
        .iter()
        .map(|range| range.clone().count())
        .sum::<usize>();

    println!("{}", ingredients);
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
        fresh_ingredient_ranges = fresh_ingredient_ranges.into_iter().fold(
            Vec::<RangeInclusive<usize>>::new(),
            |mut acc, range| {
                if let Some(last) = acc.last_mut() {
                    if *last.end() >= *range.start() - 1 {
                        *last = *last.start()..=*last.end().max(range.end());
                        return acc;
                    }
                }
                acc.push(range);
                acc
            },
        );
        let ingredients = fresh_ingredient_ranges
            .iter()
            .map(|range| range.clone().count())
            .sum::<usize>();
        assert_eq!(ingredients, 14);
    }
}
