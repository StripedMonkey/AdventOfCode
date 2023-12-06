use nom::{
    character::complete::{multispace1, u64},
    multi::separated_list1,
    IResult, sequence::terminated, bytes::complete::tag,
};
use rs_06::*;

// Time:      7  15   30
// Distance:  9  40  200
fn parse_file(file: &str) -> IResult<&str, Vec<BoatRace>> {
    let mut lines = file.lines();
    let time_line = 'time: {
        while let Some(line) = lines.next() {
            if line.starts_with("Time:") {
                break 'time line;
            }
        }
        todo!()
    };
    let (time_line, _) = terminated(tag("Time:"), multispace1)(time_line)?;
    let distance_line = 'distance: {
        while let Some(line) = lines.next() {
            if line.starts_with("Distance:") {
                break 'distance line;
            }
        }
        todo!()
    };
    let (distance_line, _) = terminated(tag("Distance:"), multispace1)(distance_line)?;

    let (_, times) = separated_list1(multispace1, u64)(time_line)?;
    let (_, distances) = separated_list1(multispace1, u64)(distance_line)?;
    Ok((
        file,
        times
            .iter()
            .zip(distances.iter())
            .map(|(time, distance)| BoatRace {
                time: *time,
                current_record: *distance,
            })
            .collect(),
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

    let answer: u64 = races.iter().map(|race| race.race()).inspect(|n| println!("{n}")).product();
    println!("Answer {answer}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_06::static_read("example1.txt");
        let races = parse_file(file).unwrap().1;

        let answer: u64 = races.iter().map(|race| race.race()).inspect(|n| println!("{n}")).product();
        println!("Answer {answer}")

    }
}
