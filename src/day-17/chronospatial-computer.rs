use std::collections::HashMap;

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
    fn from_u8(value: u8) -> Option<Self> {
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
    a: i32,
    b: i32,
    c: i32,
}

impl Computer {
    fn new(a: i32, b: i32, c: i32) -> Self {
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
            let opcode = OpCode::from_u8(self.instructions[instruction_pointer]).unwrap();
            let operand = self.instructions[instruction_pointer + 1];

            match opcode {
                OpCode::Adv => {
                    let denominator = 2_i32.pow(self.combo_value(operand, computer) as u32);
                    computer.a /= denominator;
                }
                OpCode::Bxl => {
                    computer.b ^= operand as i32;
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
                    let denominator = 2_i32.pow(self.combo_value(operand, computer) as u32);
                    computer.b = computer.a / denominator;
                }
                OpCode::Cdv => {
                    let denominator = 2_i32.pow(self.combo_value(operand, computer) as u32);
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

    fn combo_value(&self, operand: u8, computer: &Computer) -> i32 {
        match operand {
            0..=3 => operand as i32,
            4 => computer.a,
            5 => computer.b,
            6 => computer.c,
            _ => panic!("Invalid combo operand"),
        }
    }
}

fn parse_input(input: &str) -> (Computer, Vec<u8>) {
    let mut lines = input.lines();
    let mut registers = HashMap::new();

    while let Some(line) = lines.next() {
        if line.starts_with("Register") {
            let parts: Vec<&str> = line.split(':').collect();
            let key = parts[0].trim().split(' ').last().unwrap();
            let value = parts[1].trim().parse::<i32>().unwrap();
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

fn main() {
    let input = include_str!("input.data");
    let (mut computer, program_data) = parse_input(input);
    let program = Program::new(program_data);

    let output = program.run(&mut computer);
    println!("Output: {}", output);
}
