use aoc_utils::*;
use itertools::Itertools as _;
use rs_2024_15::*;

fn main() {
    let input = rs_2024_15::static_read("input1.txt");
    let (mut map, directions) = parse(&input);
    let mut starting_position = map
        .iter()
        .find_position(|row| row.contains(&'@'))
        .map(|(y, r)| (r.iter().position(|c| *c == '@').unwrap(), y))
        .unwrap();
    for direction in directions {
        (map, starting_position) = step(map, starting_position, direction);
    }
    for row in &map {
        println!("{}", row.iter().join(""));
    }
    let answer: usize = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().map(move |(x, c)| {
                if *c == '[' {
                    return (100 * y) + x;
                }
                0
            })
        })
        .flatten()
        .sum();
    println!("{}", answer);
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_15::static_read("example1.txt");
        let (mut map, directions) = parse(&input);
        let mut starting_position = map
            .iter()
            .find_position(|row| row.contains(&'@'))
            .map(|(y, r)| (r.iter().position(|c| *c == '@').unwrap(), y))
            .unwrap();

        for (i, direction) in directions.into_iter().enumerate() {
            for row in &map {
                println!("{}", row.iter().join(""));
            }
            println!("{:?}", starting_position);
            (map, starting_position) = step(map, starting_position, direction);
        }
        for row in &map {
            println!("{}", row.iter().join(""));
        }
        let answer: usize = map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().enumerate().map(move |(x, c)| {
                    if *c == '[' {
                        println!("{} {} {}", x, y, (100 * y) + x);
                        return (100 * y) + x;
                    }
                    0
                })
            })
            .flatten()
            .sum();
        assert_eq!(answer, 9021);
    }
}

fn step(
    mut map: Vec<Vec<char>>,
    starting_position: (usize, usize),
    direction: (isize, isize),
) -> (Vec<Vec<char>>, (usize, usize)) {
    let move_position = (
        starting_position.0 as isize + direction.0,
        starting_position.1 as isize + direction.1,
    );
    let move_char = map
        .get(move_position.1 as usize)
        .and_then(|row| row.get(move_position.0 as usize));
    let Some(c) = move_char else {
        return (map, starting_position);
    };
    match c {
        '.' => {
            map[starting_position.1][starting_position.0] = '.';
            map[move_position.1 as usize][move_position.0 as usize] = '@';
            (map, (move_position.0 as usize, move_position.1 as usize))
        }
        '#' => (map, starting_position),
        'O' => panic!(),
        '[' => {
            println!("Checking Moving boxes");

            if !(can_move_box(&map, move_position, direction)
                && can_move_box(&map, (move_position.0 + 1, move_position.1), direction))
            {
                return (map, starting_position);
            }
            println!("Moving boxes");
            move_box(&mut map, (move_position.0 + 1, move_position.1), direction);
            move_box(&mut map, move_position, direction);
            map[starting_position.1][starting_position.0] = '.';
            map[move_position.1 as usize][move_position.0 as usize] = '@';
            (map, (move_position.0 as usize, move_position.1 as usize))
        }
        ']' => {
            if !(can_move_box(&map, move_position, direction)
                && can_move_box(&map, (move_position.0 - 1, move_position.1), direction))
            {
                return (map, starting_position);
            }
            move_box(&mut map, (move_position.0 - 1, move_position.1), direction);
            move_box(&mut map, move_position, direction);
            map[starting_position.1][starting_position.0] = '.';
            map[move_position.1 as usize][move_position.0 as usize] = '@';
            (map, (move_position.0 as usize, move_position.1 as usize))
        }
        _ => panic!(),
    }
}

fn can_move_box(map: &Vec<Vec<char>>, position: (isize, isize), direction: (isize, isize)) -> bool {
    let pos_to_check = (position.0 + direction.0, position.1 + direction.1);
    let pos_to_check_char = map
        .get(pos_to_check.1 as usize)
        .and_then(|row| row.get(pos_to_check.0 as usize));
    let Some(c) = pos_to_check_char else {
        return false;
    };
    match c {
        '.' => true,
        '[' if direction == (-1, 0) => can_move_box(map, pos_to_check, direction),
        ']' if direction == (1, 0) => can_move_box(map, pos_to_check, direction),
        '[' => {
            can_move_box(map, pos_to_check, direction)
                && can_move_box(map, (pos_to_check.0 + 1, pos_to_check.1), direction)
        }
        ']' => {
            can_move_box(map, pos_to_check, direction)
                && can_move_box(map, (pos_to_check.0 - 1, pos_to_check.1), direction)
        }
        _ => false,
    }
}

fn move_box(map: &mut Vec<Vec<char>>, position: (isize, isize), direction: (isize, isize)) {
    let move_to = (position.0 + direction.0, position.1 + direction.1);
    let move_to_char = map
        .get(move_to.1 as usize)
        .and_then(|row| row.get(move_to.0 as usize));
    let Some(c) = move_to_char else {
        panic!();
    };
    match c {
        '[' if direction == (-1, 0) => {
            move_box(map, move_to, direction);
        }
        ']' if direction == (1, 0) => {
            move_box(map, move_to, direction);
        }
        '[' => {
            move_box(map, (move_to.0 + 1, move_to.1), direction);
            move_box(map, move_to, direction);
        }
        ']' => {
            move_box(map, (move_to.0 - 1, move_to.1), direction);
            move_box(map, move_to, direction);
        }
        '.' => {}
        _ => panic!(),
    }
    map[move_to.1 as usize][move_to.0 as usize] = map[position.1 as usize][position.0 as usize];
    map[position.1 as usize][position.0 as usize] = '.';
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(isize, isize)>) {
    let mut lines = input.lines();
    let mut map = Vec::new();
    println!("Building map");

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        map.push(
            line.chars()
                .map(|c| {
                    match c {
                        '#' => "##",
                        '.' => "..",
                        '@' => "@.",
                        'O' => "[]",
                        _ => panic!(),
                    }
                    .chars()
                })
                .flatten()
                .collect(),
        );
    }
    let mut directions = Vec::new();
    while let Some(line) = lines.next() {
        for c in line.chars() {
            match c {
                '^' => directions.push((0, -1)),
                'v' => directions.push((0, 1)),
                '<' => directions.push((-1, 0)),
                '>' => directions.push((1, 0)),
                _ => panic!(),
            }
        }
    }
    (map, directions)
}
