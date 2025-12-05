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

enum Cell {
    Empty,
    Roll,
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
    let lt_4_adj = (0..len_x)
        .cartesian_product(0..len_y)
        .filter(|&(x, y)| match &map[x][y] {
            Cell::Roll => {
                let count_adj_rolls = iter_positions((x, y), (len_x, len_y))
                    .filter(|&(adj_x, adj_y)| matches!(&map[adj_x][adj_y], Cell::Roll))
                    .count();
                count_adj_rolls < 4
            }
            Cell::Empty => false,
        })
        .count();
    println!("{}", lt_4_adj);
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
        let lt_4_adj = (0..len_x)
            .cartesian_product(0..len_y)
            .filter(|&(x, y)| match &map[x][y] {
                Cell::Roll => {
                    let count_adj_rolls = iter_positions((x, y), (len_x, len_y))
                        .filter(|&(adj_x, adj_y)| matches!(&map[adj_x][adj_y], Cell::Roll))
                        .count();
                    count_adj_rolls < 4
                }
                Cell::Empty => false,
            })
            .count();
        println!("{}", lt_4_adj);
        assert_eq!(lt_4_adj, 13);
    }
}
