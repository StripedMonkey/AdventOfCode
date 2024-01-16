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
    let result = plan.direct_volume(Clock::Clockwise);
    println!("Result: {}", result);
}

fn parse_color(color_str: &str) -> IResult<&str, &str> {
    let (color_str, _) = tag("(#")(color_str)?;
    let (color_str, color) = take(6usize)(color_str)?;
    let (color_str, _) = tag(")")(color_str)?;
    Ok((color_str, color))
}

fn parse_step(step: &str) -> IResult<&str, DigStep> {
    let (step, _) = map(one_of("UDLR"), |c| Direction::parse(c))(step)?;
    let (step, _) = tag(" ")(step)?;
    let (step, _) = map(digit1, |s| usize::from_str(s).unwrap())(step)?;
    let (step, _) = tag(" ")(step)?;
    let (step, color) = parse_color(step)?;

    let distance = usize::from_str_radix(&color[..5], 16).unwrap();
    let direction = match color.chars().nth(5).unwrap() {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!(),
    };

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
        assert_eq!(result, 952408144115);
    }
}
