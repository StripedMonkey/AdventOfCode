use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, line_ending, multispace0, multispace1, u64},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use rs_05::*;

fn main() {
    let file = *INPUT_1;
    let almanac = parse_file(file).unwrap().1;
    let smallest_seed = almanac
        .seeds_to_plant
        .iter()
        .map(|s| {
            let (mut final_type, mut final_value) = (String::from("seed"), *s);
            while let Some((s_type, s_value)) = almanac.convert(&final_type, final_value) {
                final_type = s_type.to_string();
                final_value = s_value;
            }
            (final_type, final_value)
        })
        .min_by_key(|(_, v)| *v)
        .unwrap();
    println!("The answer is {:?}", smallest_seed);}

#[derive(Debug)]
struct ConversionMap {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl ConversionMap {
    fn contains(&self, input: u64) -> bool {
        (self.source_range_start..(self.source_range_start + self.range_length)).contains(&input)
    }
    fn convert(&self, input: u64) -> u64 {
        if (self.source_range_start..(self.source_range_start + self.range_length)).contains(&input)
        {
            input - self.source_range_start + self.dest_range_start
        } else {
            input
        }
    }
}

#[derive(Debug)]
struct Almanac<'a> {
    seeds_to_plant: Vec<u64>,
    conversion_maps: HashMap<(&'a str, &'a str), Vec<ConversionMap>>,
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
}

fn parse_conversion(line: &str) -> IResult<&str, ConversionMap> {
    let (line, dest_range_start) = delimited(multispace0, u64, multispace1)(line)?;
    let (line, source_range_start) = terminated(u64, multispace1)(line)?;
    let (line, range_length) = u64(line)?;
    Ok((
        line,
        ConversionMap {
            dest_range_start,
            source_range_start,
            range_length,
        },
    ))
}

fn parse_map(file: &str) -> IResult<&str, ((&str, &str), Vec<ConversionMap>)> {
    let (file, _) = multispace0(file)?;
    let (file, map_tag) = terminated(take_till(|c: char| c.is_whitespace()), multispace1)(file)?;
    let (_, (source_tag, _, dest_tag)) = tuple((alpha1, tag("-to-"), alpha1))(map_tag)?;
    let (file, _) = tag("map:")(file)?;
    let (file, conversion_maps) = separated_list1(line_ending, parse_conversion)(file)?;
    Ok((file, ((source_tag, dest_tag), conversion_maps)))
}

fn parse_file(file: &'_ str) -> IResult<&'_ str, Almanac<'_>> {
    let (file, _) = tag("seeds: ")(file)?;
    let (file, seeds_to_plant) = separated_list1(multispace1, u64)(file)?;
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
    fn first_test() {
        let file = rs_05::static_read("example1.txt");
        let almanac = parse_file(file).unwrap().1;
        let smallest_seed = almanac
            .seeds_to_plant
            .iter()
            .map(|s| {
                let (mut final_type, mut final_value) = (String::from("seed"), *s);
                while let Some((s_type, s_value)) = almanac.convert(&final_type, final_value) {
                    final_type = s_type.to_string();
                    final_value = s_value;
                }
                (final_type, final_value)
            })
            .min_by_key(|(_, v)| *v)
            .unwrap();
        println!("The answer is {:?}", smallest_seed);
    }
}
