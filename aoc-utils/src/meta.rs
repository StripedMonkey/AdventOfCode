use std::path::Path;

pub struct DayMeta<'path> {
    day: usize,
    year: usize,
    path: &'path Path,
}

pub fn generate_meta() -> DayMeta<'static> {
    DayMeta {
        day: todo!(),
        year: todo!(),
        path: todo!(),
    }
}
