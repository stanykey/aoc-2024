use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

/// Enum for supported operators
#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    /// Parse a character into an `Operator`
    fn from_char(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Multiply),
            '|' => Some(Operator::Concatenate),
            _ => None,
        }
    }

    fn make_operator_list(operators: &str) -> Vec<Operator> {
        operators
            .chars()
            .filter_map(Operator::from_char) // Convert each character to an Operator
            .collect()
    }
}

#[derive(Debug)]
struct Equation {
    value: usize,
    operands: Vec<usize>,
}

impl Equation {
    fn parse(input: &str) -> Equation {
        let parts: Vec<&str> = input.split(':').collect();
        assert_eq!(
            parts.len(),
            2,
            "Input must contain a value and operands separated by ':'"
        );

        let value = parts[0]
            .trim()
            .parse::<usize>()
            .expect("Invalid value in input");

        let operands = parts[1]
            .trim()
            .split_whitespace()
            .map(|operand| {
                operand
                    .trim()
                    .parse::<usize>()
                    .expect("Invalid operand in input")
            })
            .collect();

        Equation { value, operands }
    }

    fn validate(&self, operators: &[Operator]) -> bool {
        let num_operators = self.operands.len() - 1;

        // Generate all possible operator combinations
        let mut combinations = (0..operators.len().pow(num_operators as u32)).map(|bitmask| {
            (0..num_operators)
                .map(|i| operators[(bitmask / operators.len().pow(i as u32)) % operators.len()])
                .collect::<Vec<Operator>>()
        });

        combinations.any(|ops| self.evaluate(&ops) == self.value)
    }

    fn evaluate(&self, operators: &[Operator]) -> usize {
        let mut result = self.operands[0];

        for (i, &operator) in operators.iter().enumerate() {
            match operator {
                Operator::Add => result += self.operands[i + 1],
                Operator::Multiply => result *= self.operands[i + 1],
                Operator::Concatenate => {
                    let rhs = self.operands[i + 1];
                    let rhs_digit_count = (rhs as f64).log10().floor() as u32 + 1;
                    result = (result * 10_usize.pow(rhs_digit_count)) + self.operands[i + 1];
                }
            }
        }

        result
    }
}

fn load_calibration_equations(file_path: &Path) -> io::Result<Vec<Equation>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| {
            let line = line?;
            Ok(Equation::parse(&line))
        })
        .collect()
}

fn get_calibration_equations(equations: &Vec<Equation>, operators: &[Operator]) -> usize {
    equations
        .iter()
        .filter(|equation| equation.validate(operators))
        .map(|equation| equation.value)
        .sum()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let calibration_equations = load_calibration_equations(file_path)?;

    let timer = Instant::now();
    let basic_operators = Operator::make_operator_list("+*");
    let calibration_value = get_calibration_equations(&calibration_equations, &basic_operators);
    println!("Calibration value (+*): {}", calibration_value);
    println!("Time elapsed: {:?}", timer.elapsed());

    let timer = Instant::now();
    let extended_operators = Operator::make_operator_list("+*|");
    let calibration_value = get_calibration_equations(&calibration_equations, &extended_operators);
    println!("Calibration value (+*|): {}", calibration_value);
    println!("Time elapsed: {:?}", timer.elapsed());

    Ok(())
}
