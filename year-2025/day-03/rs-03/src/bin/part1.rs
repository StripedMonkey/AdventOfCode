use aoc_utils::*;
use rs_2025_03::*;

fn largest_capacity(capacities: &[usize]) -> usize {
    let (idx_max, &value) = (&capacities[..capacities.len() - 1])
        .iter()
        .enumerate()
        .rev() // max_by_key selects the last max in case of ties; we want the first
        .max_by_key(|(_, &cap)| cap)
        .unwrap();
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
        println!("{}", battery_bank);

        let output = largest_capacity(&capacities);

        answer += output;
    }
    assert!(answer > 17136, "Process of elimination says this is too low");
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
            let idx_max = (&capacities[..capacities.len() - 1])
                .iter()
                .enumerate()
                .max_by_key(|(_, &cap)| cap)
                .unwrap()
                .0;
            let idx2_max = capacities
                .iter()
                .enumerate()
                .skip(idx_max + 1)
                .max_by_key(|(_, &cap)| cap)
                .unwrap()
                .0;
            let output = capacities[idx_max] * 10 + capacities[idx2_max];
            answer += output;
        }
        println!("{}", answer);
        assert_eq!(answer, 357);
    }

    #[test]
    fn test_largest_capacity() {
        let capacity = "3374246176145276565276217374117317572272168678475865167851675236237662847614486362485377284678214289";
        let output = largest_capacity(
            &capacity
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>(),
        );
    }
}
