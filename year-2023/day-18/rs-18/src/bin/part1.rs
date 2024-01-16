use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take},
    character::complete::{digit1, newline, one_of},
    combinator::map,
    IResult,
};
use rs_2023_18::*;

fn main() {
    let file = *INPUT_1;
    let (_, plan) = parse_file(file).unwrap();
    println!("Result: {}", plan.direct_volume(Clock::Clockwise)); // 46334
}

fn parse_color(color_str: &str) -> IResult<&str, &str> {
    let (color_str, _) = tag("(#")(color_str)?;
    let (color_str, rgb) = take(6usize)(color_str)?;
    let (color_str, _) = tag(")")(color_str)?;
    Ok((color_str, rgb))
}

fn parse_step(step: &str) -> IResult<&str, DigStep> {
    let (step, direction) = map(one_of("UDLR"), |c| Direction::parse(c))(step)?;
    let (step, _) = tag(" ")(step)?;
    let (step, distance) = map(digit1, |s| usize::from_str(s).unwrap())(step)?;
    let (step, _) = tag(" ")(step)?;
    let (step, _color) = parse_color(step)?;
    Ok((step, DigStep::new(direction, distance)))
}

fn parse_file(file: &str) -> IResult<&str, DigPlan> {
    let (file, steps) = nom::multi::separated_list1(newline, parse_step)(file)?;
    Ok((file, DigPlan::new(steps)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_18::static_read("example1.txt");
        let (_, plan) = parse_file(file).unwrap();
        let result = plan.direct_volume(Clock::Clockwise);
        assert_eq!(result, 62);
    }

    #[test]
    fn second_test() {
        let file = rs_2023_18::static_read("example2.txt");
        let (_, plan) = parse_file(file).unwrap();
        let result = plan.direct_volume(Clock::Counter);
        assert_eq!(result, 36);
    }
}
