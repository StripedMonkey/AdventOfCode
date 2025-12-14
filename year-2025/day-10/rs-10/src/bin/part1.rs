use nom::{
    bytes::complete::{tag, take_till},
    character::complete::space1,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};
use itertools::Itertools as _;

fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<u16>>> {
    separated_list1(
        space1,
        delimited(
            tag("("),
            separated_list1(tag(","), nom::character::complete::u16),
            tag(")"),
        ),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<bool>, Vec<Vec<u16>>, Vec<u16>)> {
    let (rest, indicator_lights) = delimited(
        tag("["),
        map(take_till(|c| c == ']'), |s: &str| {
            s.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect::<Vec<bool>>()
        }),
        tag("]"),
    )(input)?;
    let (rest, _) = space1(rest)?;
    let (rest, buttons) = parse_buttons(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, joltage_requirements) = delimited(
        tag("{"),
        separated_list1(tag(","), nom::character::complete::u16),
        tag("}"),
    )(rest)?;
    Ok((rest, (indicator_lights, buttons, joltage_requirements)))
}

struct Machine {
    indicator_lights: Vec<bool>,
    buttons: Vec<Vec<u16>>,
    joltage_requirements: Vec<u16>,
}

impl Machine {
    fn get_statemachine(&self) -> StateMachine {
        StateMachine(0)
    }

    fn get_button_masks(&self) -> Vec<u16> {
        self.buttons
            .iter()
            .map(|button| {
                button
                    .iter()
                    .fold(0u16, |acc, &light_index| acc | (1 << light_index))
            })
            .collect()
    }

    fn get_target_mask(&self) -> u16 {
        self.indicator_lights
            .iter()
            .enumerate()
            .fold(
                0u16,
                |acc, (i, &light)| {
                    if light {
                        acc | (1 << i)
                    } else {
                        acc
                    }
                },
            )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct StateMachine(u16);

impl StateMachine {
    fn press_button(&self, button_mask: u16) -> StateMachine {
        StateMachine(self.0 ^ button_mask)
    }

    fn is_target(&self, target_mask: u16) -> bool {
        self.0 == target_mask
    }
}

// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    let mut machines = Vec::new();
    for line in input.lines() {
        let (_rest, (indicator_lights, buttons, joltage_requirements)) =
            all_consuming(parse_line)(line)?;
        machines.push(Machine {
            indicator_lights,
            buttons,
            joltage_requirements,
        });
    }
    Ok(("", machines))
}

fn main() {
    let input = rs_2025_10::static_read("input1.txt");
    let (_rest, machines) = all_consuming(parse)(&input).unwrap();
    let (_rest, machines) = all_consuming(parse)(&input).unwrap();
    let mut answer = 0;
    for machine in machines {
        let state_machine = machine.get_statemachine();
        let button_masks = machine.get_button_masks();
        let target_mask = machine.get_target_mask();
        println!("Target mask: {:b}", target_mask);
        println!(
            "Button masks: {}",
            button_masks.iter().map(|m| format!("{:b}", m)).join(", ")
        );

        let shortest_path = pathfinding::directed::bfs::bfs(
            &state_machine,
            |&sm| button_masks.iter().map(move |&m| sm.press_button(m)),
            |sm| sm.is_target(target_mask),
        );
        if let Some(path) = shortest_path {
            println!("Found path of length {}", path.len() - 1);
            println!("Path: {:?}", path);
            answer += path.len() - 1;
        }
    }
    println!("Answer: {}", answer);
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools as _;
    use nom::combinator::all_consuming;

    #[test]
    fn first_test() {
        let input = rs_2025_10::static_read("example1.txt");
        let (_rest, machines) = all_consuming(parse)(&input).unwrap();
        let mut answer = 0;
        for machine in machines {
            let state_machine = machine.get_statemachine();
            let button_masks = machine.get_button_masks();
            let target_mask = machine.get_target_mask();
            println!("Target mask: {:b}", target_mask);
            println!(
                "Button masks: {}",
                button_masks.iter().map(|m| format!("{:b}", m)).join(", ")
            );

            let shortest_path = pathfinding::directed::bfs::bfs(
                &state_machine,
                |&sm| button_masks.iter().map(move |&m| sm.press_button(m)),
                |sm| sm.is_target(target_mask),
            );
            if let Some(path) = shortest_path {
                println!("Found path of length {}", path.len() - 1);
                println!("Path: {:?}", path);
                answer += path.len() - 1;
            }
        }
        println!("Answer: {}", answer);
        assert_eq!(answer, 7);
    }

    #[test]
    fn test_parse_buttons() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let (_rest, (indicator_lights, buttons, joltage_requirements)) =
            all_consuming(parse_line)(line).unwrap();
        assert_eq!(indicator_lights, vec![false, true, true, false]);
        assert_eq!(
            buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
        assert_eq!(joltage_requirements, vec![3, 5, 4, 7]);

        let machine = Machine {
            indicator_lights,
            buttons,
            joltage_requirements,
        };
        let target_mask = machine.get_target_mask();
        assert_eq!(target_mask, 0b0110);

        let button_masks = machine.get_button_masks();
        assert_eq!(
            button_masks,
            vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011]
        );
    }
}
