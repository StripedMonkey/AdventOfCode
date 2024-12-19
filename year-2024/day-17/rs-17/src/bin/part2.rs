use rs_2024_17::*;

fn main() {
    let input = rs_2024_17::static_read("input1.txt");
    let (computer, program) = parse(&input);
    let mut built_answer = 0;
    loop {
        built_answer <<= 3;
        let output = foo(&computer, &program, built_answer);
        built_answer += output as usize;
        let mut comp = computer.clone();
        comp.register_a = built_answer;
        let ans: Vec<u8> = comp.run_program(&program).unwrap();
        if ans == program {
            break;
        }
    }
    println!("Program i {built_answer}");
    println!("Program length: {:?}", program.len());}

#[derive(Debug, Clone)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
}

impl Computer {
    fn run_program(&mut self, program: &Vec<u8>) -> Option<Vec<u8>> {
        let mut program_counter = 0;
        let mut output = vec![];
        let max_steps = 500000;
        let mut steps = 0;
        loop {
            steps += 1;
            if steps > max_steps {
                return None;
            }
            let Some(&instruction) = program.get(program_counter) else {
                break;
            };
            let Some(&operand) = program.get(program_counter + 1) else {
                break;
            };
            match Instruction::from(instruction) {
                Instruction::Adv => {
                    let value = combo_operand(&self, operand);
                    let denominator = 2usize.pow(value as u32);
                    self.register_a /= denominator;
                }
                Instruction::Bxl => {
                    self.register_b ^= operand as usize;
                }
                Instruction::Bst => {
                    let value = combo_operand(&self, operand);
                    self.register_b = value % 8;
                }
                Instruction::Jnz => {
                    if self.register_a != 0 {
                        program_counter = program[program_counter + 1] as usize;
                        continue;
                    }
                }
                Instruction::Bxc => {
                    self.register_b ^= self.register_c;
                }
                Instruction::Out => {
                    let value = combo_operand(&self, operand);
                    let value = value & 0b111;
                    // if program[output.len()] != value as u8 {
                    //     return Some(output);
                    // }
                    output.push(value as u8);
                }
                Instruction::Bdv => {
                    let value = combo_operand(&self, operand);
                    let denominator = 2usize.pow(value as u32);
                    self.register_b = self.register_a / denominator;
                }
                Instruction::Cdv => {
                    let value = combo_operand(&self, operand);
                    let denominator = 2usize.pow(value as u32);
                    self.register_c = self.register_a / denominator;
                }
            }
            program_counter += 2;
        }
        Some(output)
    }

    fn find_program(&mut self, program: &Vec<u8>) -> Option<usize> {
        let mut program_counter = 0;
        let mut output = vec![];
        let max_steps = 500000;
        let mut steps = 0;
        loop {
            steps += 1;
            if steps > max_steps {
                return None;
            }
            let Some(&instruction) = program.get(program_counter) else {
                break;
            };
            let Some(&operand) = program.get(program_counter + 1) else {
                break;
            };
            match Instruction::from(instruction) {
                Instruction::Adv => {
                    let value = combo_operand(&self, operand);
                    let denominator = 2usize.pow(value as u32);
                    self.register_a /= denominator;
                }
                Instruction::Bxl => {
                    self.register_b ^= operand as usize;
                }
                Instruction::Bst => {
                    let value = combo_operand(&self, operand);
                    self.register_b = value % 8;
                }
                Instruction::Jnz => {
                    if self.register_a != 0 {
                        program_counter = program[program_counter + 1] as usize;
                        continue;
                    }
                }
                Instruction::Bxc => {
                    self.register_b ^= self.register_c;
                }
                Instruction::Out => {
                    let value = combo_operand(&self, operand);
                    let value = value & 0b111;
                    if program[output.len()] != value as u8 {
                        return Some(output.len());
                    }
                    output.push(value as u8);
                }
                Instruction::Bdv => {
                    let value = combo_operand(&self, operand);
                    let denominator = 2usize.pow(value as u32);
                    self.register_b = self.register_a / denominator;
                }
                Instruction::Cdv => {
                    let value = combo_operand(&self, operand);
                    let denominator = 2usize.pow(value as u32);
                    self.register_c = self.register_a / denominator;
                }
            }
            program_counter += 2;
        }
        Some(output.len())
    }
}

enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

fn combo_operand(computer: &Computer, value: u8) -> usize {
    match value {
        v @ 0..=3 => v as usize,
        4 => computer.register_a,
        5 => computer.register_b,
        6 => computer.register_c,
        _ => panic!("Invalid Combo Operand"),
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use rayon::iter::{IndexedParallelIterator as _, IntoParallelIterator as _, ParallelIterator as _};

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_17::static_read("example2.txt");
        let (mut computer, program) = parse(&input);
        let answer = (1523076922204280..1523076922204280 + 100)
            .into_par_iter()
            .find_first(|&i| {
                let mut computer = computer.clone();
                computer.register_a = i;
                let Some(output) = computer.run_program(&program) else {
                    return false;
                };
                if output == program {
                    println!("{:?}", i);
                    return true;
                }
                false
            });

        println!("{:?}", answer);
    }

    #[test]
    fn how_size_influences_length() {
        let input = rs_2024_17::static_read("input1.txt");
        let (computer, program) = parse(&input);
        let answer: Vec<_> = (0..64)
            .into_par_iter()
            .map(|i| usize::MAX >> i)
            .map(|i| {
                let mut computer = computer.clone();
                computer.register_a = i;
                let Some(output) = computer.run_program(&program) else {
                    return 0;
                };
                output.len()
            })
            .collect();
        // 16, 16, 16, 15, 15, 15, 14, 14, 14, 13, 13, 13, 12, 12, 12, 11, 11, 11, 10, 10, 10, 9, 9, 9, 8, 8, 8, 7, 7, 7, 6, 6, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 1, 1, 1
        println!("Program length: {:?}", program.len());
        println!("{:?}", answer);
    }

    #[test]
    fn in_range() {
        let input = rs_2024_17::static_read("input1.txt");
        let (computer, program) = parse(&input);
        let answer = ((2usize.pow(45))..(2usize.pow(48)))
            .into_par_iter()
            .by_exponential_blocks()
            .find_first(|&i| {
                let mut computer = computer.clone();
                computer.register_a = i;
                let Some(output) = computer.run_program(&program) else {
                    return false;
                };
                if output == program {
                    println!("{:?}", i);
                    return true;
                }
                false
            });

        println!("Program length: {:?}", program.len());
        println!("{:?}", answer);
    }

    #[test]
    fn matches() {
        let input = rs_2024_17::static_read("input1.txt");
        let (computer, program) = parse(&input);
        let mut built_answer = 0;
        loop {
            built_answer <<= 3;
            let previous = built_answer;
            let output = foo(&computer, &program, built_answer);
            built_answer += output as usize;
            let diff = built_answer - previous;
            println!("{:?} diff {}", built_answer, diff);
            let mut comp = computer.clone();
            comp.register_a = built_answer;
            let ans: Vec<u8> = comp.run_program(&program).unwrap();
            if ans == program {
                println!("{:?}", built_answer);
                break;
            }
        }
        println!("Program i {built_answer}");
        println!("Program length: {:?}", program.len());
    }

    #[test]
    fn asdf() {
        let input = rs_2024_17::static_read("input1.txt");
        let (mut computer, program) = parse(&input);

        let i = 1523076922204280;
        computer.register_a = i;
        let Some(output) = computer.run_program(&program) else {
            panic!();
        };
        assert_eq!(output, program);
    }
}

fn foo(computer: &Computer, program: &Vec<u8>, value: usize) -> u32 {
    for i in 0..u16::MAX {
        let mut computer = computer.clone();
        computer.register_a = value + i as usize;
        let Some(output) = computer.run_program(&program) else {
            panic!();
        };
        if i % 1000 == 0 {
            println!("P: {:?} O: {:?}", program, output);
        }
        if program.len() < output.len() {
            continue;
        }
        let compared = &program[program.len() - output.len()..];
        if compared == output {
            return i as u32;
        }
    }
    todo!()
}

fn parse(input: &str) -> (Computer, Vec<u8>) {
    let mut lines = input.lines();

    let register_a = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .parse()
        .unwrap();
    let register_b = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .parse()
        .unwrap();
    let register_c = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .parse()
        .unwrap();

    let computer = Computer {
        register_a,
        register_b,
        register_c,
    };
    lines.next();
    let program = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    (computer, program)
}
