use itertools::Itertools as _;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator as _};
use rs_2025_10::parse;
use z3::ast::Int;

fn main() {
    let input = rs_2025_10::static_read("input1.txt");

    let (_rest, machines) = parse(&input).unwrap();
    let mut answer = 0;

    machines.iter().for_each(|machine| {
        // Using Z3
        let cfg = z3::Config::new();
        z3::with_z3_config(&cfg, || {
            let optimizer = z3::Optimize::new();
            let target = machine.get_target_joltage();
            let target = target
                .into_iter()
                .map(|v| Int::from_u64(v as u64))
                .collect_vec();
            let buttons = machine.get_buttons_joltage();
            let times_pressed = (0..buttons.len())
                .map(|i| Int::new_const(format!("btn_{}", i)))
                .collect_vec();

            for tp in &times_pressed {
                optimizer.assert(&tp.ge(&Int::from_u64(0)));
            }

            for (joltage_idx, target_joltage) in target.iter().enumerate() {
                let sum = Int::add(
                    &buttons
                        .iter()
                        .zip(times_pressed.iter())
                        .filter_map(|(btn, tp)| {
                            if btn.contains(&(joltage_idx as u16)) {
                                return Some(tp);
                            }
                            None
                        })
                        .collect_vec(),
                );
                optimizer.assert(&sum.eq(target_joltage));
            }

            optimizer.minimize(&Int::add(&times_pressed));

            match optimizer.check(&[]) {
                z3::SatResult::Sat => {
                    let model = optimizer.get_model().unwrap();
                    let total_presses: u32 = times_pressed
                        .iter()
                        .map(|tp| model.eval(tp, true).unwrap().as_u64().unwrap() as u32)
                        .sum();
                    println!("Total presses: {}", total_presses);
                    answer += total_presses;
                }
                z3::SatResult::Unsat => {
                    panic!("No solution found");
                }
                z3::SatResult::Unknown => {
                    panic!("Unknown result");
                }
            }
        });
    });

    println!("Answer: {}", answer);
    assert_eq!(answer, 16474);
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools as _;
    use nom::combinator::all_consuming;
    use z3::ast::Int;

    #[test]
    fn first_test() {
        let input = rs_2025_10::static_read("example1.txt");
        let (_rest, machines) = parse(&input).unwrap();
        let mut answer = 0;

        machines.iter().for_each(|machine| {
            // Using Z3
            let cfg = z3::Config::new();
            z3::with_z3_config(&cfg, || {
                let optimizer = z3::Optimize::new();
                let target = machine.get_target_joltage();
                let target = target
                    .into_iter()
                    .map(|v| Int::from_u64(v as u64))
                    .collect_vec();
                let buttons = machine.get_buttons_joltage();
                let times_pressed = (0..buttons.len())
                    .map(|i| Int::new_const(format!("btn_{}", i)))
                    .collect_vec();

                for tp in &times_pressed {
                    optimizer.assert(&tp.ge(&Int::from_u64(0)));
                }

                for (joltage_idx, target_joltage) in target.iter().enumerate() {
                    let sum = Int::add(
                        &buttons
                            .iter()
                            .zip(times_pressed.iter())
                            .filter_map(|(btn, tp)| {
                                if btn.contains(&(joltage_idx as u16)) {
                                    return Some(tp);
                                }
                                None
                            })
                            .collect_vec(),
                    );
                    optimizer.assert(&sum.eq(target_joltage));
                }

                optimizer.minimize(&Int::add(&times_pressed));

                match optimizer.check(&[]) {
                    z3::SatResult::Sat => {
                        let model = optimizer.get_model().unwrap();
                        let total_presses: u32 = times_pressed
                            .iter()
                            .map(|tp| model.eval(tp, true).unwrap().as_u64().unwrap() as u32)
                            .sum();
                        println!("Total presses: {}", total_presses);
                        answer += total_presses;
                    }
                    z3::SatResult::Unsat => {
                        panic!("No solution found");
                    }
                    z3::SatResult::Unknown => {
                        panic!("Unknown result");
                    }
                }
            });
        });

        println!("Answer: {}", answer);
        assert_eq!(answer, 33);
    }
}
