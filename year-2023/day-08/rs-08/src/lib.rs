use std::{collections::HashMap, env, fmt, path::PathBuf, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
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

fn parse_node(line: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (line, node_name) = alphanumeric1(line)?;
    let (line, _) = tag(" = (")(line)?;
    let (line, left) = alphanumeric1(line)?;
    let (line, _) = tag(", ")(line)?;
    let (line, right) = alphanumeric1(line)?;
    let (line, _) = tag(")")(line)?;
    Ok((line, (node_name, (left, right))))
}

pub fn parse_network(lines: &str) -> IResult<&str, NetworkNav> {
    let (lines, directions) = many1(map(one_of("LR"), |a| a.try_into().unwrap()))(lines)?;
    let (lines, _) = newline(lines)?;
    let (lines, _) = newline(lines)?;
    let (lines, nodes) = separated_list1(newline, parse_node)(lines)?;
    let nodes = nodes.into_iter().collect();
    Ok((lines, NetworkNav { directions, nodes }))
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryInto<Direction> for char {
    type Error = ();

    fn try_into(self) -> Result<Direction, Self::Error> {
        match self {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct NetworkNav<'a> {
    directions: Vec<Direction>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl NetworkNav<'_> {
    pub fn walk<'a>(
        &'a self,
        starting: &'a str,
    ) -> impl Iterator<Item = (usize, &str)> + fmt::Debug {
        NavWalker {
            network: self,
            current_node: starting,
            current_direction: 0,
        }
    }

    pub fn starting_nodes<'a>(&'a self, starting: &'a str) -> impl Iterator<Item = &str> + 'a {
        self.nodes.iter().filter_map(move |node| {
            if node.0.ends_with(starting) {
                Some(*node.0)
            } else {
                None
            }
        })
    }

    pub fn find_cycle<'a>(&'a self, starting: &'a str) -> Cycle {
        let cycle_walker = NavWalker {
            network: self,
            current_node: starting,
            current_direction: 0,
        };
        let mut past_steps = Vec::new();
        let mut last_node = None;
        for (step, node) in cycle_walker {
            if past_steps.contains(&(step, node)) {
                last_node = Some((step, node));
                break;
            }
            past_steps.push((step, node));
        }
        let Some(last_node) = last_node else {
            panic!();
        };
        // past_steps.push(last_node);
        let cycle_length = past_steps.iter().skip_while(|e| *e != &last_node).count();
        let cycle_start = past_steps.len() - cycle_length;
        Cycle {
            length: cycle_length,
            start: cycle_start,
            steps: past_steps,
        }
    }
}

pub struct NavWalker<'a> {
    network: &'a NetworkNav<'a>,
    current_node: &'a str,
    current_direction: usize,
}

#[derive(Debug)]
pub struct Cycle<'a> {
    pub length: usize,
    pub start: usize,
    pub steps: Vec<(usize, &'a str)>,
}

impl<'a> Iterator for NavWalker<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some((
            self.current_direction % self.network.directions.len(),
            self.current_node,
        ));
        let (left, right) = self.network.nodes.get(self.current_node)?;
        let next_node = match self
            .network
            .directions
            .get(self.current_direction % self.network.directions.len())?
        {
            Direction::Left => left,
            Direction::Right => right,
        };
        self.current_node = next_node;
        self.current_direction += 1;
        ret
    }
}

impl fmt::Debug for NavWalker<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NavWalker")
            .field("current_node", &self.current_node)
            .field(
                "current_direction",
                &self.network.directions.get(self.current_direction),
            )
            .finish()
    }
}
