use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, u64},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use rs_2023_06::*;

// Time:      7  15   30
// Distance:  9  40  200
fn parse_file(file: &str) -> IResult<&str, Vec<BoatRace>> {
    let mut lines = file.lines();
    let time_line = lines.next().unwrap();
    let (time_line, _) = terminated(tag("Time:"), multispace1)(time_line)?;
    let distance_line = lines.next().unwrap();
    let (distance_line, _) = terminated(tag("Distance:"), multispace1)(distance_line)?;

    let time = time_line
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>();
    let distance = distance_line
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>();
    Ok((
        file,
        vec![BoatRace {
            time: time.unwrap(),
            current_record: distance.unwrap(),
        }],
    ))
}
struct BoatRace {
    time: u64,
    current_record: u64,
}

impl BoatRace {
    fn race(&self) -> u64 {
        return (0..self.time - 1)
            .filter_map(|hold_time| {
                let distance = hold_time * (self.time - hold_time);
                if distance > self.current_record {
                    Some(hold_time)
                } else {
                    None
                }
            })
            .count() as u64;
    }
}

fn main() {
    let file = *INPUT_1;
    let races = parse_file(file).unwrap().1;

    let answer: u64 = races
        .iter()
        .map(|race| race.race())
        .inspect(|n| println!("{n}"))
        .product();
    println!("Answer {answer}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_06::static_read("example1.txt");
        let races = parse_file(file).unwrap().1;

        let answer: u64 = races
            .iter()
            .map(|race| race.race())
            .inspect(|n| println!("{n}"))
            .product();
        println!("Answer {answer}")
    }
}
