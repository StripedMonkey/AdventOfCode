use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive}, cmp::min,
};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, line_ending, multispace0, multispace1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rs_05::*;

fn main() {
    let file = *INPUT_1;
    let almanac = &parse_file(file).unwrap().1;
    let smallest_seed = (&almanac
        .seeds_to_plant)
        .iter()
        .flat_map(|s| {
            let current_range = s
                .clone().into_par_iter()
                .map(|seed| {
                    let (mut final_type, mut final_value) = ("seed", seed);
                    while let Some((s_type, s_value)) = almanac.convert(&final_type, final_value) {
                        final_type = s_type;
                        final_value = s_value;
                    }
                    (final_type, final_value)
                })
                .min();
            println!("Range min: {current_range:?}");
            current_range
        })
        .min_by_key(|(_, v)| *v)
        .unwrap();
    println!("The answer is {:?}", smallest_seed);
}

#[derive(Debug)]
struct ConversionRange {
    source_range: RangeInclusive<u64>,
    dest_range: RangeInclusive<u64>,
}

type ConvertedAndRemainder = (
    Option<RangeInclusive<u64>>,
    Option<Vec<RangeInclusive<u64>>>,
);

impl ConversionRange {
    fn contains(&self, input: u64) -> bool {
        self.source_range.contains(&input)
    }
    fn convert(&self, input: u64) -> u64 {
        if self.source_range.contains(&input) {
            input - self.source_range.start() + self.dest_range.start()
        } else {
            input
        }
    }

    // When converting a range, there are five possible outcomes:
    // 1. Source range is a subset of the input range, meaning the ends of the input range are unconverted
    //    Input:  -----------------
    //    Source:    ----------
    // 2. Input range is a subset of the input range, meaning that the entire range may be converted
    //    Input:     ----------
    //    Source: -----------------
    // 3. The start of the input range is before the source range, meaning that the start is unconverted
    //    Input:   ------------------
    //    Source:      -------------------
    // 4. The end of the input range is after the end of the source range, meaning that the end is unconverted
    //    Input:       ------------------
    //    Source:  -----------------
    // 5. The ranges are completely disjoint, meaning that the entire range is unconverted
    //    Input:              --------------
    //    Source:  --------
    fn convert_range(&self, input: RangeInclusive<u64>) -> ConvertedAndRemainder {
        // Input range is a subset of the source range
        let mut remainder = Vec::new();
        if input.start() < self.source_range.start() {
            remainder.push(*input.start()..=*min(&(self.source_range.start() - 1), input.end()));
        }
        if input.end() < self.source_range.end() {
            remainder.push(*input.start()..=*min(&(self.source_range.start() - 1), input.end()));
        }
        // The ranges are completely disjoint
        (None, Some(remainder))
    }
}

#[derive(Debug)]
struct Almanac<'a> {
    seeds_to_plant: Vec<RangeInclusive<u64>>,
    conversion_maps: HashMap<(&'a str, &'a str), Vec<ConversionRange>>,
}

impl Almanac<'_> {
    fn convert(&self, input_type: &str, input: u64) -> Option<(&str, u64)> {
        let Some(key) = self.conversion_maps.keys().find(|(a, _)| *a == input_type) else {
            return None;
        };
        let conversion_maps = self.conversion_maps.get(key).unwrap();
        for map in conversion_maps {
            if map.contains(input) {
                return Some((key.1, map.convert(input)));
            }
        }
        Some((key.1, input))
    }

    fn convert_range(&self, input_type: &str, input: Range<u64>) -> Option<Vec<Range<u64>>> {
        let Some(key) = self.conversion_maps.keys().find(|(a, _)| *a == input_type) else {
            return None;
        };
        let conversion_maps = self.conversion_maps.get(key).unwrap();
        for map in conversion_maps {
            if map.contains(input.start) && map.contains(input.end) {
                return Some(vec![map.convert(input.start)..map.convert(input.end)]);
            }
            if map.contains(input.start) {
                return Some(vec![map.convert(input.start)..input.end]);
            }
        }
        None
    }
}

fn parse_conversion(line: &str) -> IResult<&str, ConversionRange> {
    let (line, dest_range_start) = delimited(multispace0, u64, multispace1)(line)?;
    let (line, source_range_start) = terminated(u64, multispace1)(line)?;
    let (line, range_length) = u64(line)?;
    Ok((
        line,
        ConversionRange {
            source_range: source_range_start..=(source_range_start + range_length),
            dest_range: dest_range_start..=(dest_range_start + range_length),
        },
    ))
}

fn parse_map(file: &str) -> IResult<&str, ((&str, &str), Vec<ConversionRange>)> {
    let (file, _) = multispace0(file)?;
    let (file, map_tag) = terminated(take_till(|c: char| c.is_whitespace()), multispace1)(file)?;
    let (_, (source_tag, _, dest_tag)) = tuple((alpha1, tag("-to-"), alpha1))(map_tag)?;
    let (file, _) = tag("map:")(file)?;
    let (file, conversion_maps) = separated_list1(line_ending, parse_conversion)(file)?;
    Ok((file, ((source_tag, dest_tag), conversion_maps)))
}

fn parse_file(file: &'_ str) -> IResult<&'_ str, Almanac<'_>> {
    let (file, _) = tag("seeds: ")(file)?;
    let (file, seeds_to_plant) = separated_list1(
        multispace1,
        map(tuple((u64, multispace0, u64)), |(start, _, len)| {
            start..=(start + len)
        }),
    )(file)?;
    let (file, x) = separated_list1(line_ending, parse_map)(file)?;
    Ok((
        file,
        Almanac {
            seeds_to_plant,
            conversion_maps: x.into_iter().collect(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn second_test() {
        let file = rs_05::static_read("example1.txt");
        let almanac = parse_file(file).unwrap().1;
        let smallest_seed = almanac
            .seeds_to_plant
            .iter()
            .flat_map(|s| {
                s.clone()
                    .map(|seed| {
                        let (mut final_type, mut final_value) = ("seed", seed);
                        while let Some((s_type, s_value)) =
                            almanac.convert(&final_type, final_value)
                        {
                            final_type = s_type;
                            final_value = s_value;
                        }
                        (final_type, final_value)
                    })
                    .min()
            })
            .min_by_key(|(_, v)| *v)
            .unwrap();
        println!("The answer is {:?}", smallest_seed);
    }

    #[test]
    fn range_test() {
        let conv = ConversionRange {
            source_range: 1..=5,
            dest_range: 15..=20,
        };
        println!("{:?}", conv.convert_range(3..=5)); // Completely within the source range
        println!("{:?}", conv.convert_range(0..=6)); // ends are outside of source range, should have range 0 to 0, and 6 to 6 remaining
        println!("{:?}", conv.convert_range(0..=2)); // Start is outside of source range, should have range 0 to 0 remaining
        println!("{:?}", conv.convert_range(4..=7)); // End is outside of source range, should have range 6 to 7 remaining
    }
}
