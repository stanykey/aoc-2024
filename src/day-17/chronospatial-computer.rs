use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
enum OpCode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl OpCode {
    fn new(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Adv),
            1 => Some(Self::Bxl),
            2 => Some(Self::Bst),
            3 => Some(Self::Jnz),
            4 => Some(Self::Bxc),
            5 => Some(Self::Out),
            6 => Some(Self::Bdv),
            7 => Some(Self::Cdv),
            _ => None,
        }
    }
}

struct Computer {
    a: i64,
    b: i64,
    c: i64,
}

impl Computer {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Self { a, b, c }
    }
}

struct Program {
    instructions: Vec<u8>,
}

impl Program {
    fn new(instructions: Vec<u8>) -> Self {
        Self { instructions }
    }

    fn run(&self, computer: &mut Computer) -> String {
        let mut instruction_pointer = 0;
        let mut output = Vec::new();

        while instruction_pointer < self.instructions.len() {
            let opcode = OpCode::new(self.instructions[instruction_pointer]).unwrap();
            let operand = self.instructions[instruction_pointer + 1];

            match opcode {
                OpCode::Adv => {
                    let denominator = 2_i64.pow(self.combo_value(operand, computer) as u32);
                    computer.a /= denominator;
                }
                OpCode::Bxl => {
                    computer.b ^= operand as i64;
                }
                OpCode::Bst => {
                    let value = self.combo_value(operand, computer) % 8;
                    computer.b = value;
                }
                OpCode::Jnz => {
                    if computer.a != 0 {
                        instruction_pointer = operand as usize;
                        continue;
                    }
                }
                OpCode::Bxc => {
                    computer.b ^= computer.c;
                }
                OpCode::Out => {
                    let value = self.combo_value(operand, computer) % 8;
                    output.push(value);
                }
                OpCode::Bdv => {
                    let denominator = 2_i64.pow(self.combo_value(operand, computer) as u32);
                    computer.b = computer.a / denominator;
                }
                OpCode::Cdv => {
                    let denominator = 2_i64.pow(self.combo_value(operand, computer) as u32);
                    computer.c = computer.a / denominator;
                }
            }

            instruction_pointer += 2;
        }

        output
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn combo_value(&self, operand: u8, computer: &Computer) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4 => computer.a,
            5 => computer.b,
            6 => computer.c,
            _ => panic!("Invalid combo operand"),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .instructions
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{}", str)
    }
}

fn parse_input(input: &str) -> (Computer, Vec<u8>) {
    let mut lines = input.lines();
    let mut registers = HashMap::new();

    while let Some(line) = lines.next() {
        if line.starts_with("Register") {
            let parts: Vec<&str> = line.split(':').collect();
            let key = parts[0].trim().split(' ').last().unwrap();
            let value = parts[1].trim().parse::<i64>().unwrap();
            registers.insert(key, value);
        } else if line.starts_with("Program") {
            let program = line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split(',')
                .map(|x| x.trim().parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            return (
                Computer::new(
                    *registers.get("A").unwrap(),
                    *registers.get("B").unwrap(),
                    *registers.get("C").unwrap(),
                ),
                program,
            );
        }
    }
    panic!("Invalid input format");
}

fn simulate_single_iteration(mut reg_a: i64, program: &[u8]) -> u8 {
    let mut reg_b = 0;
    let mut reg_c = 0;
    let mut output = 0;

    for pair in program[..(program.len() - 2)].chunks_exact(2) {
        let op = pair[0];
        let param = if op == 1 {
            pair[1] as i64
        } else if pair[1] == 4 {
            reg_a
        } else if pair[1] == 5 {
            reg_b
        } else if pair[1] == 6 {
            reg_c
        } else {
            pair[1] as i64
        };

        match op {
            0 => reg_a >>= param,
            1 => reg_b ^= param,
            2 => reg_b = param % 8,
            4 => reg_b ^= reg_c,
            5 => output = param,
            6 => reg_b = reg_a >> param,
            7 => reg_c = reg_a >> param,
            op => panic!("Invalid opcode {}", op),
        }
    }

    (output % 8) as u8
}

fn find_register_a(current_a: i64, program: &[u8], output_index: usize) -> Option<i64> {
    for bit in 0..8 {
        let reg_a = (current_a << 3) | bit;
        let result = simulate_single_iteration(reg_a, program);

        if result != program[output_index] {
            continue;
        }

        if output_index == 0 {
            return Some(reg_a);
        } else if let Some(reg_a) = find_register_a(reg_a, program, output_index - 1) {
            return Some(reg_a);
        }
    }

    None
}

fn find_lowest_register_a(program: &Program) -> i64 {
    // I gave up with found generic non-brute solution and found one for investigation at
    // https://github.com/smith61/advent_of_code/blob/main/src/year_2024/day_17.rs
    find_register_a(0, &program.instructions, program.instructions.len() - 1).unwrap_or(0)
}

fn main() {
    let input = include_str!("input.data");
    let (mut computer, program_data) = parse_input(input);
    let program = Program::new(program_data);

    let output = program.run(&mut computer);
    println!("Output: {}", output);

    println!(
        "The lowest positive value for register A is: {}",
        find_lowest_register_a(&program)
    );
}

#[cfg(test)]
mod tests {
    use crate::{find_lowest_register_a, Computer, Program};

    #[test]
    fn test_part_one_example() {
        let mut computer = Computer::new(729, 0, 0);
        let program = Program::new(vec![0, 1, 5, 4, 3, 0]);
        let output = program.run(&mut computer);
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_two_basic() {
        let mut computer = Computer::new(117440, 0, 0);
        let program = Program::new(vec![0, 3, 5, 4, 3, 0]);
        let output = program.run(&mut computer);
        assert_eq!(output, program.to_string());
    }

    #[test]
    fn test_find_lowest_register_a() {
        let program = Program::new(vec![0, 3, 5, 4, 3, 0]);
        assert_eq!(find_lowest_register_a(&program), 117440);
    }
}
