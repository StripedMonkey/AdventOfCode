use std::{env, path::PathBuf, str::FromStr};

use crate::meta::DayMeta;

pub mod cycles;
mod meta;
mod text;

lazy_static::lazy_static! {
    pub static ref DAY_META: DayMeta<'static> = meta::generate_meta();
}

pub fn static_read(file_path: &str) -> &'static str {
    let mut cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).expect("Infallible action failed!");
    cwd.pop();
    let file_path = cwd.join(file_path);
    let file = std::fs::read_to_string(file_path).expect("Failed to open file!");
    Box::leak(file.into_boxed_str())
}
