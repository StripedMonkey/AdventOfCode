use std::{path::PathBuf, str::FromStr};

use lazy_static::lazy_static;

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

pub struct Boxes<'a> {
    boxes: Vec<Vec<Lens<'a>>>,
}

#[derive(Debug, Clone)]
pub struct Lens<'a> {
    label: &'a str,
    focal_len: usize,
}

pub enum Operation<'a> {
    Add(Lens<'a>),
    Remove(Lens<'a>),
}

impl<'a> Boxes<'a> {
    pub fn new() -> Self {
        Self {
            boxes: vec![Vec::new(); 256],
        }
    }
    fn add<'b: 'a>(&mut self, lens: Lens<'b>) {
        let hash = lens.hash();
        let lenses = &mut self.boxes[hash];
        if let Some(old_lens) = lenses.iter_mut().find(|l| l.label == lens.label) {
            old_lens.focal_len = lens.focal_len;
        } else {
            lenses.push(lens);
        }
    }
    fn remove(&mut self, lens: Lens) {
        let hash = lens.hash();
        let lenses = &mut self.boxes[hash];
        // remove the lens with the label
        lenses.retain(|l| l.label != lens.label);
    }

    pub fn extend<'b: 'a>(&mut self, iter: impl Iterator<Item = Operation<'b>>) {
        iter.for_each(|op| match op {
            Operation::Add(lens) => self.add(lens),
            Operation::Remove(lens) => self.remove(lens),
        })
    }

    pub fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(move |(box_num, lens_box)| {
                lens_box
                    .iter()
                    .enumerate()
                    .map(move |(slot, lens)| lens.focal_len * (box_num + 1) * (slot + 1))
            })
            .flatten()
            .sum()
    }
}

impl Lens<'_> {
    fn new(label: &str, focal_len: usize) -> Lens<'_> {
        Lens { label, focal_len }
    }
    fn hash(&self) -> usize {
        hash_str(self.label)
    }
}

pub fn hash_str(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| 17 * (acc + c as usize) % 256)
}

fn parse_operation(lens: &str) -> Operation<'_> {
    if lens.contains("=") {
        let (label, focal_len) = lens.split_once("=").unwrap();
        Operation::Add(Lens::new(label, focal_len.parse::<usize>().unwrap()))
    } else if lens.contains("-") {
        let (label, _) = lens.split_once("-").unwrap();
        Operation::Remove(Lens::new(label, 0))
    } else {
        panic!("Not a valid lens")
    }
}

pub fn parse_file(file: &str) -> impl Iterator<Item = Operation<'_>> {
    split_operations(file).map(|op| parse_operation(op))
}

pub fn split_operations(file: &str) -> impl Iterator<Item = &str> {
    file.lines().map(|line| line.split(",")).flatten()
}
