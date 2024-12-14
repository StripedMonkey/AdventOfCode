
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
            let Some((A, B)) = intersection(slope_a, slope_b, target) else {
                return 0;
            };
            println!("A: {}, B: {}", A, B);
            A * 3 + B
        })
        .sum::<usize>();
    println!("{:?}", result);
    assert!(result == 28753);
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
                let Some((A, B)) = intersection(slope_a, slope_b, target) else {
                    return 0;
                };
                // println!("A: {}, B: {} -> {}", A, B, A * 3 + B);
                // println!("A: {}, B: {} -> {}", A, B, A * 3 + B);
                A * 3 + B
            })
            .sum::<usize>();
        println!("{:?}", result);
        assert!(result == 480);
    }
}

#[derive(Debug)]
struct ArcadeMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

