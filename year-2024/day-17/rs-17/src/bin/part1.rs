use aoc_utils::*;
use itertools::Itertools as _;
use rs_2024_17::*;

fn main() {
    let input = rs_2024_17::static_read("input1.txt");
    let (mut computer, program) = parse(&input);

    let output = computer.run_program(&program);
    println!("{:?}", computer);
    println!("{:?}", program);
    println!("{:?}", output.iter().join(","));
}

#[derive(Debug)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
}

impl Computer {
    fn run_program(&mut self, program: &Vec<u8>) -> Vec<usize>{
        let mut program_counter = 0;
        let mut output = vec![];
        loop {
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
                    output.push(value & 0b111);
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
        output
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

    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_17::static_read("example1.txt");
        let (mut computer, program) = parse(&input);

        let output = computer.run_program(&program);
        println!("{:?}", computer);
        println!("{:?}", program);
        println!("{:?}", output);
        assert!(output == vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn instruction_operations_1() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 0,
            register_c: 9,
        };
        let program = vec![2,6];
        computer.run_program(&program);
        assert!(computer.register_b == 1);
    }

    #[test]
    fn instruction_operations_2() {
        let mut computer = Computer {
            register_a: 10,
            register_b: 0,
            register_c: 0,
        };
        let program = vec![5,0,5,1,5,4];
        let output = computer.run_program(&program);
        assert!(output == vec![0,1,2]);
    }
    #[test]
    fn instruction_operations_3() {
        let mut computer = Computer {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
        };
        let program = vec![0,1,5,4,3,0];
        let output = computer.run_program(&program);
        assert!(output == vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert!(computer.register_a == 0);
    }
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
