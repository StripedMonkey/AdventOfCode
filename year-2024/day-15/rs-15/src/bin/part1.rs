use aoc_utils::*;
use rs_2024_15::*;
use itertools::Itertools as _;

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
    let answer: usize = map.iter().enumerate().map(|(y,row)| {
        row.iter().enumerate().map(move |(x, c)| {
            if *c == 'O' {
                println!("{} {} {}", x, y,(100 * y) + x);
                return (100 * y) + x
            }
            0
        })
    }).flatten().sum();
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
        for direction in directions {
            (map, starting_position) = step(map, starting_position, direction);
        }
        for row in &map {
            println!("{}", row.iter().join(""));
        }
        let answer: usize = map.iter().enumerate().map(|(y,row)| {
            row.iter().enumerate().map(move |(x, c)| {
                if *c == 'O' {
                    println!("{} {} {}", x, y,(100 * y) + x);
                    return (100 * y) + x
                }
                0
            })
        }).flatten().sum();
        assert_eq!(answer, 10092);
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
        'O' => {
            let mut box_end = move_position;
            loop {
                box_end = (box_end.0 + direction.0, box_end.1 + direction.1);
                let box_end_char = map
                    .get(box_end.1 as usize)
                    .and_then(|row| row.get(box_end.0 as usize));
                let Some(c) = box_end_char else {
                    break (map, starting_position);
                };
                match c {
                    '.' => {
                        map[starting_position.1][starting_position.0] = '.';
                        map[move_position.1 as usize][move_position.0 as usize] = '@';
                        map[box_end.1 as usize][box_end.0 as usize] = 'O';
                        return (map, (move_position.0 as usize, move_position.1 as usize));
                    }
                    '#' => break (map, starting_position),
                    'O' => continue,
                    _ => panic!(),
                }
            }
        }
        _ => panic!(),
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(isize, isize)>) {
    let mut lines = input.lines();
    let mut map = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        map.push(line.chars().collect());
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
