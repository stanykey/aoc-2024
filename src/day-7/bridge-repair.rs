use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

    fn validate(&self) -> bool {
        let num_operators = self.operands.len() - 1;
        let mut combinations = (0..2_usize.pow(num_operators as u32)).map(|bitmask| {
            (0..num_operators)
                .map(|i| if (bitmask & (1 << i)) != 0 { '*' } else { '+' })
                .collect::<Vec<char>>()
        });

        combinations.any(|ops| self.evaluate(&ops) == self.value)
    }

    fn evaluate(&self, operators: &[char]) -> usize {
        let mut result = self.operands[0];

        for (i, &operator) in operators.iter().enumerate() {
            match operator {
                '+' => result += self.operands[i + 1],
                '*' => result *= self.operands[i + 1],
                _ => panic!("Unsupported operator"),
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

fn get_calibration_equations(equations: &Vec<Equation>) -> usize {
    equations
        .iter()
        .filter(|equation| equation.validate())
        .map(|equation| equation.value)
        .sum()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let calibration_equations = load_calibration_equations(file_path)?;
    let calibration_value = get_calibration_equations(&calibration_equations);
    println!("Calibration value: {}", calibration_value);

    Ok(())
}
