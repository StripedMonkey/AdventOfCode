use itertools::Itertools;
use rs_2025_04::*;

fn iter_positions(
    position: (usize, usize),
    positive_boundary: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let positions: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    positions.iter().filter_map(move |(dx, dy)| {
        if position.0 == 0 && *dx == -1 {
            return None;
        }
        if position.1 == 0 && *dy == -1 {
            return None;
        }
        let new_x = (position.0 as isize + dx) as usize;
        let new_y = (position.1 as isize + dy) as usize;
        if new_x >= positive_boundary.0 {
            return None;
        }
        if new_y >= positive_boundary.1 {
            return None;
        }
        Some((new_x, new_y))
    })
}

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Roll,
}

fn can_remove(map: &Vec<Vec<Cell>>, position: (usize, usize)) -> bool {
    let len_x = map.len();
    let len_y = map[0].len();
    let (x, y) = position;
    match &map[x][y] {
        Cell::Roll => {
            let count_adj_rolls = iter_positions((x, y), (len_x, len_y))
                .filter(|&(adj_x, adj_y)| matches!(&map[adj_x][adj_y], Cell::Roll))
                .count();
            count_adj_rolls < 4
        }
        Cell::Empty => false,
    }
}
fn main() {
    let input = rs_2025_04::static_read("input1.txt");
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll,
                    _ => panic!("Unknown cell type"),
                })
                .collect()
        })
        .collect::<Vec<Vec<Cell>>>();
    let len_x = map.len();
    let len_y = map[0].len();
    let mut working_map = map.clone();
    let mut rolls_removed = 0;
    loop {
        let lt_4_adj: Vec<_> = (0..len_x)
            .cartesian_product(0..len_y)
            .filter(|&(x, y)| can_remove(&working_map, (x, y)))
            .collect();
        if lt_4_adj.is_empty() {
            break;
        }
        rolls_removed += lt_4_adj.len();
        for (x, y) in lt_4_adj {
            working_map[x][y] = Cell::Empty;
        }
    }
    println!("{}", rolls_removed);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2025_04::static_read("example1.txt");
        let map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Cell::Empty,
                        '@' => Cell::Roll,
                        _ => panic!("Unknown cell type"),
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Cell>>>();
        let len_x = map.len();
        let len_y = map[0].len();
        let mut working_map = map.clone();
        let mut rolls_removed = 0;
        loop {
            let lt_4_adj: Vec<_> = (0..len_x)
                .cartesian_product(0..len_y)
                .filter(|&(x, y)| can_remove(&working_map, (x, y)))
                .collect();
            if lt_4_adj.is_empty() {
                break;
            }
            rolls_removed += lt_4_adj.len();
            for (x, y) in lt_4_adj {
                working_map[x][y] = Cell::Empty;
            }
        }
        println!("{}", rolls_removed);
        assert_eq!(rolls_removed, 43);
    }
}
