use aoc_utils::*;
use rs_2024_04::*;

fn main() {
    let input = rs_2024_04::static_read("input1.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut starting_points = Vec::new();
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, cell)| {
            if *cell == 'X' {
                starting_points.push((i, j));
            }
        })
    });
    let directions: Vec<(isize, isize)> = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let answer: usize = starting_points
        .iter()
        .map(|(i, j)| {
            let mut found_lines = 0;
            'dir: for (dx, dy) in &directions {
                let (mut current_x, mut current_y) = (*i as isize, *j as isize);
                for c in ['M', 'A', 'S'] {
                    let Some(x) = current_x.checked_add(*dx) else {
                        continue 'dir;
                    };
                    let Some(y) = current_y.checked_add(*dy) else {
                        continue 'dir;
                    };
                    if y >= grid.len() as isize || x >= grid[0].len() as isize {
                        continue 'dir;
                    }
                    if x < 0 || y < 0 {
                        continue 'dir;
                    }

                    if grid[x as usize][y as usize] != c {
                        continue 'dir;
                    }
                    current_x = x;
                    current_y = y;
                }
                found_lines += 1;
            }
            found_lines
        })
        .sum();
    println!("{}", answer);

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_04::static_read("example1.txt");
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut starting_points = Vec::new();
        grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, cell)| {
                if *cell == 'X' {
                    starting_points.push((i, j));
                }
            })
        });
        let directions: Vec<(isize, isize)> = vec![
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        let answer: usize = starting_points
            .iter()
            .map(|(i, j)| {
                let mut found_lines = 0;
                'dir: for (dx, dy) in &directions {
                    let (mut current_x, mut current_y) = (*i as isize, *j as isize);
                    for c in ['M', 'A', 'S'] {
                        let Some(x) = current_x.checked_add(*dx) else {
                            continue 'dir;
                        };
                        let Some(y) = current_y.checked_add(*dy) else {
                            continue 'dir;
                        };
                        if y >= grid.len() as isize || x >= grid[0].len() as isize {
                            continue 'dir;
                        }
                        if x < 0 || y < 0 {
                            continue 'dir;
                        }

                        if grid[x as usize][y as usize] != c {
                            continue 'dir;
                        }
                        current_x = x;
                        current_y = y;
                    }
                    found_lines += 1;
                }
                found_lines
            })
            .sum();
        println!("{}", answer);
        assert!(answer == 18);
    }
}
