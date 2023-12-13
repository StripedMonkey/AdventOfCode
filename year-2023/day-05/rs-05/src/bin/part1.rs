use rs_05::*;

fn main() {
    let file = *INPUT_1;
    let (seeds_to_plant, almanac) = parse_seed_file(file).unwrap().1;
    let smallest_seed = seeds_to_plant
        .into_iter()
        .filter_map(|seed| almanac.map_seed("seed", "location", seed))
        .min()
        .unwrap();
    println!("The answer is {:?}", smallest_seed); // 462648396
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_05::static_read("example1.txt");
        let (seeds_to_plant, almanac) = parse_seed_file(file).unwrap().1;
        let smallest_seed = seeds_to_plant
            .into_iter()
            .filter_map(|seed| almanac.map_seed("seed", "location", seed))
            .min()
            .unwrap();
        assert_eq!(smallest_seed, 35);
        println!("The answer is {:?}", smallest_seed);
    }
}
