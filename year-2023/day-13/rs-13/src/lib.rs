use std::{fmt::Display,env, path::PathBuf, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    multi::{many1, separated_list1},
    IResult,
};

#[macro_use]
extern crate lazy_static;

// The input texts are static, should it be? Probably not, but it was an excuse to do it this way.
lazy_static!{
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

fn parse_grid(input: &str) -> IResult<&str, MirrorInput> {
    let (input, grid) = separated_list1(tag("\n"), many1(one_of(".#")))(input)?;
    Ok((input, MirrorInput { grid }))
}

pub fn parse_file(input: &str) -> IResult<&str, Vec<MirrorInput>> {
    let (input, grids) = separated_list1(tag("\n\n"), parse_grid)(input)?;
    Ok((input, grids))
}

pub struct MirrorInput {
    grid: Vec<Vec<char>>,
}

impl Display for MirrorInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl MirrorInput {
    /// Returns the number of columns to the left of each vertical line and the horizontal lines above
    pub fn puzzle(&self) -> (Option<usize>, Option<usize>) {
        let (vertical, horizontal) = (self.find_vertical(), self.find_horizontal());
        assert!(vertical.is_some() ^ horizontal.is_some());
        (vertical, horizontal)
    }

    pub fn puzzle_fuzzy(&self) -> (Option<usize>, Option<usize>) {
        let (vertical, horizontal) = (self.fudge_vertical(), self.fudge_horizontal());
        assert!(vertical.is_some() ^ horizontal.is_some());
        (vertical, horizontal)
    }

    fn fudgeable_symmetric<const FUDGEABLE: bool, F>(len: usize, check_equal: F) -> Option<usize>
    where
        F: Fn(usize, usize, &mut bool) -> bool,
    {
        let mut mirrorpoint = 0;
        'check_mirror: loop {
            let mut can_fudge = FUDGEABLE;
            let centerpoint: usize = mirrorpoint;
            if centerpoint >= len - 1 {
                return None;
            }
            let outer_edge = centerpoint.min(len - centerpoint - 2);
            for offset in (0..=outer_edge).rev() {
                if !check_equal(
                    centerpoint - offset,
                    centerpoint + offset + 1,
                    &mut can_fudge,
                ) {
                    mirrorpoint += 1;
                    continue 'check_mirror;
                }
            }
            if can_fudge {
                mirrorpoint += 1;
                continue 'check_mirror;
            }
            return Some(mirrorpoint + 1);
        }
    }

    fn find_vertical(&self) -> Option<usize> {
        let len = self.grid[0].len();
        Self::fudgeable_symmetric::<false, _>(len, |l, r, _| self.column_eq(l, r))
    }

    fn find_horizontal(&self) -> Option<usize> {
        let len = self.grid.len();
        Self::fudgeable_symmetric::<false, _>(len, |l, r, _| self.grid[l] == self.grid[r])
    }

    fn fudge_vertical(&self) -> Option<usize> {
        let width = self.grid[0].len();
        Self::fudgeable_symmetric::<true, _>(width, |l, r, c| self.fudge_column_eq(l, r, c))
    }

    fn fudge_horizontal(&self) -> Option<usize> {
        let height = self.grid.len();
        Self::fudgeable_symmetric::<true, _>(height, |l, r, c| self.fudge_row_eq(l, r, c))
    }

    fn fudge_iter_all<It1, It2, I>(a: It1, b: It2, can_fudge: &mut bool) -> bool
    where
        It1: Iterator<Item = I>,
        It2: Iterator<Item = I>,
        I: Eq,
    {
        let mut attempted_fudge = !*can_fudge;
        let fudged_equal = a.zip(b).all(|(l, r)| {
            if l != r && !attempted_fudge {
                attempted_fudge = true;
                return true;
            }
            l == r
        });
        if fudged_equal && attempted_fudge {
            *can_fudge = false;
        }
        fudged_equal
    }

    fn column_eq(&self, bottom_row: usize, top_row: usize) -> bool {
        if top_row >= self.grid[0].len() {
            panic!()
        }
        self.fudge_column_eq(bottom_row, top_row, &mut false)
    }
    fn fudge_column_eq(&self, bottom_row: usize, top_row: usize, can_fudge: &mut bool) -> bool {
        if top_row >= self.grid[0].len() {
            panic!();
        }
        let bottom = self.grid.iter().map(|row| row[bottom_row]);
        let top = self.grid.iter().map(|row| row[top_row]);
        Self::fudge_iter_all(bottom, top, can_fudge)
    }

    fn fudge_row_eq(&self, top: usize, bottom: usize, can_fudge: &mut bool) -> bool {
        if bottom >= self.grid.len() {
            panic!()
        }
        let left = self.grid[top].iter();
        let right = self.grid[bottom].iter();
        Self::fudge_iter_all(left, right, can_fudge)
    }
}
