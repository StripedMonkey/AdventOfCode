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

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn parse(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
    fn rotate(&self, clock: Clock) -> Direction {
        match (self, clock) {
            (Direction::Up, Clock::Clockwise) | (Direction::Down, Clock::Counter) => {
                Direction::Right
            }
            (Direction::Up, Clock::Counter) | (Direction::Down, Clock::Clockwise) => {
                Direction::Left
            }
            (Direction::Left, Clock::Counter) | (Direction::Right, Clock::Clockwise) => {
                Direction::Down
            }
            (Direction::Left, Clock::Clockwise) | (Direction::Right, Clock::Counter) => {
                Direction::Up
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Color(u8, u8, u8);

// R 6 (#70c710)
#[derive(Debug, Clone)]
pub struct DigStep {
    direction: Direction,
    distance: usize,
}

impl DigStep {
    pub fn new(direction: Direction, distance: usize) -> DigStep {
        DigStep {
            direction,
            distance,
        }
    }
}

pub struct DigPlan {
    steps: Vec<DigStep>,
}

impl DigPlan {
    pub fn new(steps: Vec<DigStep>) -> DigPlan {
        DigPlan { steps }
    }
    fn calculate_boundary(&self) -> ((usize, usize), (usize, usize)) {
        let (mut x, mut y) = (0i32, 0i32);
        let (mut max_x, mut max_y) = (0, 0);
        let (mut min_x, mut min_y) = (0, 0);
        for step in &self.steps {
            let distance = step.distance;
            match step.direction {
                Direction::Up => y -= distance as i32,
                Direction::Left => x -= distance as i32,
                Direction::Down => y += distance as i32,
                Direction::Right => x += distance as i32,
            }
            (max_x, max_y) = (max_x.max(x), max_y.max(y));
            (min_x, min_y) = (min_x.min(x), min_y.min(y));
        }
        let offset = ((min_x.abs()) as usize, (min_y.abs()) as usize);
        let size = ((max_x - min_x) as usize, (max_y - min_y) as usize);
        (size, offset)
    }

    pub fn direct_volume(&self, start: Clock) -> usize {
        let (_size, offset) = self.calculate_boundary();
        let (mut current_x, mut current_y) = offset;
        let mut volume_x: isize = 0;
        let mut normal_dir = self.steps[0].direction.rotate(start);
        let mut previous_direction = self.steps.last().unwrap().direction;
        let mut iter = self.steps.iter().peekable();
        while let Some(current_step) = iter.next() {
            let current_rotation = clock_dir(previous_direction, current_step.direction);
            let next_rotation = clock_dir(
                current_step.direction,
                iter.peek()
                    .copied()
                    .or_else(|| Some(&self.steps[0]))
                    .unwrap()
                    .direction,
            );
            let dx = {
                match (current_rotation, next_rotation) {
                    (Clock::Clockwise, Clock::Clockwise) => match normal_dir {
                        Direction::Up | Direction::Down => 1,
                        Direction::Left | Direction::Right => 0,
                    },
                    (Clock::Clockwise, Clock::Counter) | (Clock::Counter, Clock::Clockwise) => 0,
                    (Clock::Counter, Clock::Counter) => match normal_dir {
                        Direction::Up | Direction::Down => -1,
                        Direction::Left | Direction::Right => 0,
                    },
                }
            };

            let distance = current_step.distance.checked_add_signed(dx).unwrap();
            let current_volume = match current_step.direction {
                Direction::Left => -(distance as isize * (current_y + 1) as isize),
                Direction::Right => (distance) as isize * (current_y) as isize,
                _ => 0,
            };
            volume_x += current_volume;
            if matches!(current_step.direction, Direction::Left | Direction::Right) {
                println!(
                    "P: ({current_x},{current_y}) -> {distance} {direction:?} ({current_volume}dv) {clock:?} ({external} external)",
                    distance = current_step.distance,
                    direction = current_step.direction,
                    clock = current_rotation,
                    external = distance,
                );
                println!("Current Volume {volume_x}");
            }
            (current_x, current_y) = match current_step.direction {
                Direction::Up => (current_x, current_y - current_step.distance),
                Direction::Down => (current_x, current_y + current_step.distance),
                Direction::Left => (current_x - current_step.distance, current_y),
                Direction::Right => (current_x + current_step.distance, current_y),
            };
            normal_dir = normal_dir.rotate(next_rotation);
            previous_direction = current_step.direction;
        }
        println!("Xv:{volume_x}");
        volume_x.abs() as usize
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Clock {
    Clockwise,
    Counter,
}

fn clock_dir(previous: Direction, current: Direction) -> Clock {
    match (previous, current) {
        (Direction::Right, Direction::Down) => Clock::Clockwise,
        (Direction::Down, Direction::Left) => Clock::Clockwise,
        (Direction::Left, Direction::Up) => Clock::Clockwise,
        (Direction::Up, Direction::Right) => Clock::Clockwise,
        (Direction::Left, Direction::Down) => Clock::Counter,
        (Direction::Down, Direction::Right) => Clock::Counter,
        (Direction::Right, Direction::Up) => Clock::Counter,
        (Direction::Up, Direction::Left) => Clock::Counter,
        _ => panic!(),
    }
}
