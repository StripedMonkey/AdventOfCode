use lazy_static::lazy_static;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, multispace1, newline, one_of, u64},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    ops::RangeInclusive,
    path::PathBuf,
    str::FromStr,
};

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

#[derive(Debug)]
pub struct Rule<'a> {
    pub accept: &'a str,
    pub cmp: Ordering,
    pub value: usize,
    pub next: &'a str,
}

type Part<'a> = HashMap<&'a str, usize>;
type PartRestrictions<'a> = HashMap<&'a str, RangeInclusive<usize>>;

#[derive(Debug)]
pub struct System<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part<'a>>,
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    next: &'a str,
}

impl Workflow<'_> {
    fn apply(&self, part: &Part) -> &str {
        for rule in &self.rules {
            if let Some(category) = part.get(rule.accept) {
                if category.cmp(&rule.value) == rule.cmp {
                    return rule.next;
                }
            }
        }
        return self.next;
    }
}
impl System<'_> {
    pub fn get_accepted(&self) -> impl Iterator<Item = &Part<'_>> {
        self.parts.iter().filter(|part| self.process_part(part))
    }

    fn process_part(&self, part: &Part) -> bool {
        let mut current_workflow = "in";
        loop {
            if current_workflow == "A" {
                return true;
            }
            if current_workflow == "R" {
                return false;
            }

            let Some(workflow) = self.workflows.get(current_workflow) else {
                panic!("Workflow {current_workflow} not found");
            };
            match workflow.apply(part) {
                "A" => return true,
                "R" => return false,
                next => current_workflow = next,
            }
        }
    }

    pub fn possible(&self) -> usize {
        // let mut accepted_ranges = Vec::new();
        let mut total_combinations: usize = 0;
        let mut current_ranges = VecDeque::new();
        let part_restrictions: PartRestrictions = vec![
            ("x", 1..=4000),
            ("m", 1..=4000),
            ("a", 1..=4000),
            ("s", 1..=4000),
        ]
        .into_iter()
        .collect();
        current_ranges.push_back(("in", part_restrictions));
        'workflows: while let Some((current_workflow, restrictions)) = current_ranges.pop_front() {
            if current_workflow == "A" {
                // accepted_ranges.push(restrictions);
                let mut combinations = 1;
                for v in restrictions.values() {
                    combinations *= v.end() - v.start() + 1;
                }
                total_combinations += combinations;
                continue;
            }
            if current_workflow == "R" {
                continue;
            }
            let Some(workflow) = self.workflows.get(current_workflow) else {
                panic!()
            };
            for rule in &workflow.rules {
                if let Some(allowed_range) = restrictions.get(rule.accept) {
                    if allowed_range.contains(&rule.value) {
                        match rule.cmp {
                            Ordering::Less if *allowed_range.start() == rule.value => continue,
                            Ordering::Greater if *allowed_range.end() == rule.value => continue,
                            _ => {}
                        }
                        split_allowed(&mut current_ranges, restrictions, rule, current_workflow);
                        continue 'workflows;
                    }
                }
            }
            current_ranges.push_back((workflow.next, restrictions));
        }
        total_combinations
    }
}

fn split_allowed<'a>(
    range_queue: &mut VecDeque<(&'a str, PartRestrictions<'a>)>,
    restrictions: PartRestrictions<'a>,
    rule: &Rule<'a>,
    current_workflow: &'a str,
) {
    match rule.cmp {
        Ordering::Less => {
            let mut lower = restrictions.clone();
            lower.get_mut(rule.accept).map(|range| {
                assert!(range.start() < range.end());
                *range = *range.start()..=rule.value - 1;
            });
            range_queue.push_back((rule.next, lower));
            let mut upper = restrictions.clone();
            upper.get_mut(rule.accept).map(|range| {
                assert!(range.start() < range.end());
                *range = rule.value..=*range.end();
            });
            range_queue.push_back((current_workflow, upper));
        }
        Ordering::Greater => {
            let mut lower = restrictions.clone();
            lower.get_mut(rule.accept).map(|range| {
                assert!(range.start() < range.end());
                *range = *range.start()..=rule.value;
            });
            range_queue.push_back((current_workflow, lower));
            let mut upper = restrictions.clone();
            upper.get_mut(rule.accept).map(|range| {
                assert!(range.start() < range.end());
                *range = rule.value + 1..=*range.end();
            });
            range_queue.push_back((rule.next, upper));
        }
        Ordering::Equal => todo!(),
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule<'_>> {
    let (input, rating) = alphanumeric1(input)?;
    let (input, cmp) = one_of("<>")(input)?;
    let (input, num) = u64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, next) = alphanumeric1(input)?;
    Ok((
        input,
        Rule {
            accept: rating,
            cmp: match cmp {
                '<' => Ordering::Less,
                '>' => Ordering::Greater,
                _ => panic!(),
            },
            value: num as usize,
            next,
        },
    ))
}

fn parse_workflow(input: &str) -> IResult<&str, (&str, Workflow)> {
    let (input, name) = alphanumeric1(input)?;
    let (input, (rules, next)) = delimited(
        char('{'),
        separated_pair(
            separated_list1(char(','), parse_rule),
            char(','),
            alphanumeric1,
        ),
        char('}'),
    )(input)?;
    Ok((input, (name, Workflow { rules, next })))
}

fn parse_part(input: &str) -> IResult<&str, HashMap<&str, usize>> {
    let (input, list) =
        separated_list1(char(','), separated_pair(alphanumeric1, char('='), u64))(input)?;
    Ok((
        input,
        list.into_iter().map(|(n, u)| (n, u as usize)).collect(),
    ))
}

pub fn parse_file(file: &str) -> IResult<&str, System<'_>> {
    let (file, workflows) = separated_list1(newline, parse_workflow)(file)?;
    let (file, _) = multispace1(file)?;
    let (file, parts) =
        separated_list1(newline, delimited(char('{'), parse_part, char('}')))(file)?;
    Ok((
        file,
        System {
            workflows: workflows.into_iter().collect(),
            parts,
        },
    ))
}
