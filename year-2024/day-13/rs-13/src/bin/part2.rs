
use rs_2024_13::*;

fn main() {
    let input = rs_2024_13::static_read("input1.txt");
    let machines = parse(input);
    let result = machines
        .iter()
        .map(|m| {
            let slope_a = m.button_a;
            let slope_b = m.button_b;
            let target = m.prize;
            let Some((A, B)) = intersection(
                slope_a,
                slope_b,
                (10000000000000 + target.0, 10000000000000 + target.1),
            ) else {
                return 0;
            };
            println!("A: {}, B: {}", A, B);
            (A as usize * 3) + B as usize
        })
        .sum::<usize>();
    println!("{:?}", result);
    assert!(result == 102718967795500);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_13::static_read("example1.txt");
        let machines = parse(input);
        let result = machines
            .iter()
            .map(|m| {
                let slope_a = m.button_a;
                let slope_b = m.button_b;
                let target = m.prize;
                let Some((A, B)) = intersection(
                    slope_a,
                    slope_b,
                    (10000000000000 + target.0, 10000000000000 + target.1),
                ) else {
                    return 0;
                };
                // println!("A: {}, B: {} -> {}", A, B, A * 3 + B);
                A * 3 + B
            })
            .sum::<usize>();
        println!("{:?}", result);
    }
}

