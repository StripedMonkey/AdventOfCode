use aoc_utils::*;
use rs_2025_03::*;

fn largest(capacities: &[usize]) -> (usize, usize) {
    let (idx_max, &value) = (&capacities[..capacities.len() - 1])
        .iter()
        .enumerate()
        .rev() // max_by_key selects the last max in case of ties; we want the first
        .max_by_key(|(_, &cap)| cap)
        .unwrap();
    (idx_max, value)
}

fn recurse(capacities: &[usize], depth: usize) -> usize {
    let (idx_max, &value) = capacities[..capacities.len() - depth]
        .iter()
        .enumerate()
        .rev() // max_by_key selects the last max in case of ties; we want the first
        .max_by_key(|(_, &cap)| cap)
        .unwrap();
    if depth == 0 {
        return value;
    }
    let remaining = recurse(&capacities[idx_max + 1..], depth - 1);
    return (10usize.pow(depth as u32) * value) + remaining;
}

fn largest_capacity(capacities: &[usize]) -> usize {
    let (idx_max, value) = largest(&capacities[..capacities.len() - 1]);

    assert!(capacities[..idx_max].iter().all(|&cap| cap < value));
    let (idx2_max, &value2) = capacities
        .iter()
        .enumerate()
        .skip(idx_max + 1)
        .max_by_key(|(_, &cap)| cap)
        .unwrap();

    assert!(capacities[idx2_max..].iter().all(|&cap| cap <= value2));
    assert!(value >= value2 || idx2_max == capacities.len() - 1);

    value * 10 + value2
}

fn main() {
    let input = rs_2025_03::static_read("input1.txt");
    let mut answer = 0;
    for battery_bank in input.lines() {
        if battery_bank.trim().is_empty() {
            continue;
        }
        let capacities = battery_bank
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();

        let output = recurse(&capacities, 11);
        answer += output;
    }
    println!("{}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2025_03::static_read("example1.txt");
        let mut answer = 0;
        for battery_bank in input.lines() {
            if battery_bank.trim().is_empty() {
                continue;
            }
            let capacities = battery_bank
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>();

            let output = recurse(&capacities, 11);
            answer += output;
        }
        println!("{}", answer);
        assert_eq!(answer, 3121910778619);
    }

    #[test]
    fn test_largest_capacity() {
        let capacity = "987654321111111";
        let capacities = capacity
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();

        let output = recurse(&capacities, 11);
        assert_eq!(output, 987654321111);
    }
}
