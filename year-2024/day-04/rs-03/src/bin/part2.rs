use aoc_utils::*;
use rs_2024_04::*;

fn main() {
    let input = rs_2024_04::static_read("input1.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut starting_points = Vec::new();
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, cell)| {
            if *cell == 'A' {
                starting_points.push((i, j));
            }
        })
    });
    let directions: Vec<(isize, isize)> = vec![
        (1, 1),
        (1, -1),
    ];
    let answer: usize = starting_points
        .iter().filter(|(i,j)| {
            !(*i == 0 || *j == 0 || *i == grid.len() - 1 || *j == grid[0].len() - 1)
        })
        .map(|(i, j)| {
            // starting in one of the direction, go opposite and see if you get m a s
            let mut found_Xs = 0;
            for (dx, dy) in &directions {
                let first = grid[(*i as isize + dx) as usize][(*j as isize + dy) as usize];
                let second = grid[(*i as isize - dx) as usize][(*j as isize - dy) as usize];
                if !((first == 'M' && second == 'S') || (first == 'S' && second == 'M')) {
                    return 0;
                }
            }
            1
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
                if *cell == 'A' {
                    starting_points.push((i, j));
                }
            })
        });
        let directions: Vec<(isize, isize)> = vec![
            (1, 1),
            (1, -1),
        ];
        let answer: usize = starting_points
            .iter().filter(|(i,j)| {
                !(*i == 0 || *j == 0 || *i == grid.len() - 1 || *j == grid[0].len() - 1)
            })
            .map(|(i, j)| {
                // starting in one of the direction, go opposite and see if you get m a s
                let mut found_Xs = 0;
                for (dx, dy) in &directions {
                    let first = grid[(*i as isize + dx) as usize][(*j as isize + dy) as usize];
                    let second = grid[(*i as isize - dx) as usize][(*j as isize - dy) as usize];
                    if !((first == 'M' && second == 'S') || (first == 'S' && second == 'M')) {
                        return 0;
                    }
                }
                1
            })
            .sum();
        println!("{}", answer);
        assert!(answer == 9)
    }
}
