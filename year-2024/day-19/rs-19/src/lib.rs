use std::{path::PathBuf, str::FromStr as _};

#[macro_use]
extern crate lazy_static;


// Read a file path relative to the parent of the current day's project directory.
// Each day contains the inputs and examples for that day in the the parent, so I can
// (in theory/later) share the inputs between multiple languages. Pretend like I'll actually do that.
pub fn static_read(file_path: &str) -> &'static str {
    let mut cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).expect("Infallible action failed!");
    cwd.pop();
    let file_path = cwd.join(file_path);
    println!("{:?}", file_path);
    let file = std::fs::read_to_string(file_path).expect("Failed to open file!");
    Box::leak(file.into_boxed_str())
}

pub fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();
    let available_towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    lines.next();
    let desired_patterns = lines.map(|s| s.to_string()).collect();
    (available_towels, desired_patterns)
}
