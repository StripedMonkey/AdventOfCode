use std::iter::once;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rs_2023_16::*;

fn main() {
    let file = *INPUT_1;
    let map = parse_file(&file);

    let column_starts = (0..map.width())
        .map(|x| {
            once((Direction::Down, (x, 0))).chain(once((Direction::Up, (x, map.height() - 1))))
        })
        .flatten();
    let row_starts = (0..map.height())
        .map(|y| {
            once((Direction::Right, (0, y))).chain(once((Direction::Left, (map.width() - 1, y))))
        })
        .flatten();
    let starts = column_starts.chain(row_starts).collect_vec();
    let result = starts
        .into_par_iter()
        .map(|start| map.traverse(start))
        .max()
        .unwrap();
    println!("The answer is {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_16::static_read("example1.txt");
        let map = parse_file(&file);

        let column_starts = (0..map.width()).map(|x| {
            vec![
                (Direction::Down, (x, 0)),
                (Direction::Up, (x, map.height() - 1)),
            ]
        });
        let row_starts = (0..map.height()).map(|y| {
            vec![
                (Direction::Right, (0, y)),
                (Direction::Left, (map.width() - 1, y)),
            ]
        });
        let result = column_starts
            .chain(row_starts)
            .flatten()
            .map(|start| map.traverse(start))
            .max()
            .unwrap();
        println!("The answer is {}", result);
        assert_eq!(result, 51);
    }
}
