use std::{collections::HashMap, env, ops::Range, path::PathBuf, str::FromStr};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, line_ending, multispace0, multispace1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[macro_use]
extern crate lazy_static;

// The input texts are static, should it be? Probably not, but it was an excuse to do it this way.
lazy_static! {
    pub static ref INPUT_1: &'static str = static_read("input1.txt");
    pub static ref INPUT_2: &'static str = static_read("input2.txt");
}

// Read a file path relative to the parent of the current day's project directory.
// Each day contains the inputs and examples for that day in the the parent, so I can
// (in theory/later) share the inputs between multiple languages. Pretend like I'll actually do that.
pub fn static_read(file_path: &str) -> &'static str {
    let mut cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).expect("Infallible action failed!");
    cwd.pop();
    let file_path = cwd.join(file_path);
    let file = std::fs::read_to_string(file_path).expect("Failed to open file!");
    Box::leak(file.into_boxed_str())
}

fn parse_conversion(line: &str) -> IResult<&str, ConversionRange> {
    let (line, dest_range_start) = delimited(multispace0, u64, multispace1)(line)?;
    let (line, source_range_start) = terminated(u64, multispace1)(line)?;
    let (line, range_length) = u64(line)?;
    Ok((
        line,
        ConversionRange {
            input: source_range_start..(source_range_start + range_length),
            output: dest_range_start..(dest_range_start + range_length),
        },
    ))
}

fn parse_map(file: &str) -> IResult<&str, ((&str, &str), Vec<ConversionRange>)> {
    let (file, _) = multispace0(file)?;
    let (file, map_tag) = terminated(take_till(|c: char| c.is_whitespace()), multispace1)(file)?;
    let (_, (source_tag, _, dest_tag)) = tuple((alpha1, tag("-to-"), alpha1))(map_tag)?;
    let (file, _) = tag("map:")(file)?;
    let (file, conversion_map) = separated_list1(line_ending, parse_conversion)(file)?;
    Ok((file, ((source_tag, dest_tag), conversion_map)))
}

pub fn parse_seed_file(
    file: &'_ str,
) -> IResult<&'_ str, (Vec<u64>, SeedAlmanac<'_, ConversionRange>)> {
    let (file, _) = tag("seeds: ")(file)?;
    let (file, seeds_to_plant) = separated_list1(multispace1, u64)(file)?;
    let (file, x) = separated_list1(line_ending, parse_map)(file)?;
    Ok((
        file,
        (
            seeds_to_plant,
            SeedAlmanac {
                conversion_maps: x.into_iter().collect(),
            },
        ),
    ))
}
pub fn parse_seed_range_file(
    file: &'_ str,
) -> IResult<&'_ str, (Vec<Range<u64>>, SeedAlmanac<'_, ConversionRange>)> {
    let (file, _) = tag("seeds: ")(file)?;
    let (file, seeds_to_plant) = separated_list1(
        multispace1,
        map(tuple((u64, multispace0, u64)), |(start, _, len)| {
            start..(start + len)
        }),
    )(file)?;
    let (file, x) = separated_list1(line_ending, parse_map)(file)?;
    Ok((
        file,
        (
            seeds_to_plant,
            SeedAlmanac {
                conversion_maps: x.into_iter().collect(),
            },
        ),
    ))
}

pub trait Convertable {
    fn contains(&self, input: u64) -> bool;
    fn convert(&self, input: u64) -> u64;
}

#[derive(Debug)]
pub struct SeedAlmanac<'a, C>
where
    C: Convertable,
{
    conversion_maps: HashMap<(&'a str, &'a str), Vec<C>>,
}

#[derive(Debug)]
pub struct ConversionRange {
    input: Range<u64>,
    output: Range<u64>,
}

impl Convertable for ConversionRange {
    fn contains(&self, input: u64) -> bool {
        self.input.contains(&input)
    }
    fn convert(&self, input: u64) -> u64 {
        if self.input.contains(&input) {
            input - self.input.start + self.output.start
        } else {
            input
        }
    }
}

impl<C> SeedAlmanac<'_, C>
where
    C: Convertable,
{
    pub fn map_seed(&self, input_type: &str, output_type: &str, input: u64) -> Option<u64> {
        let (mut final_type, mut final_value) = (input_type, input);
        while let Some((s_type, s_value)) = self.convert_seed(&final_type, final_value) {
            if s_type == output_type {
                return Some(s_value);
            }
            (final_type, final_value) = (s_type, s_value);
        }
        None
    }

    pub fn convert_seed(&self, input_type: &str, input: u64) -> Option<(&str, u64)> {
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
