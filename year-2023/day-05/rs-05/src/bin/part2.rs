use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rs_05::*;

fn main() {
    let file = *INPUT_1;
    let (seeds_to_plant, almanac) = parse_seed_range_file(file).unwrap().1;
    let smallest_seed = seeds_to_plant
        .into_iter()
        .flat_map(|s| {
            s.into_par_iter()
                .map(|seed| almanac.map_seed("seed", "location", seed))
                .min()
        })
        .flatten()
        .min()
        .unwrap();
    println!("The answer is {:?}", smallest_seed);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn second_test() {
        let file = rs_05::static_read("example1.txt");
        let (seeds_to_plant, almanac) = parse_seed_range_file(file).unwrap().1;
        let smallest_seed = seeds_to_plant
            .into_iter()
            .flat_map(|s| {
                s.clone()
                    .map(|seed| almanac.map_seed("seed", "location", seed))
                    .min()
            })
            .flatten()
            .min()
            .unwrap();
        assert_eq!(smallest_seed, 46);
        println!("The answer is {:?}", smallest_seed);
    }

    // #[test]
    // fn range_test() {
    //     let conv = ConversionRange {
    //         input: 1..5,
    //         output: 15..20,
    //     };
    //     println!("{:?}", conv.convert_range(3..5)); // Completely within the source range
    //     assert_eq!(conv.convert_range(1..5), (Some(15..19), None));
    //     println!("{:?}", conv.convert_range(0..6)); // ends are outside of source range, should have range 0 to 0, and 6 to 6 remaining
    //     println!("{:?}", conv.convert_range(0..2)); // Start is outside of source range, should have range 0 to 0 remaining
    //     println!("{:?}", conv.convert_range(4..7)); // End is outside of source range, should have range 6 to 7 remaining
    // }
}
