use regex::Regex;
use std::path::Path;
use std::{fs, io};

enum Instruction {
    Mul(i32, i32),
}

fn load_program_source_code(file_path: &Path) -> io::Result<String> {
    let result = fs::read_to_string(file_path);
    match result {
        Ok(text) => Ok(text.trim().to_string()),
        Err(e) => Err(e),
    }
}

fn compile(source_code: &str) -> Vec<Instruction> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut operations = Vec::new();
    for cap in regex.captures_iter(source_code) {
        if let (Ok(lhs), Ok(rhs)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
            operations.push(Instruction::Mul(lhs, rhs));
        }
    }
    operations
}

fn compile_with_reenabling_feature(source_code: &str) -> Vec<Instruction> {
    let regex = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(don't)\(\)|(do)\(\)").unwrap();

    let mut operations = Vec::new();
    let mut skip = false;
    for cap in regex.captures_iter(source_code) {
        if let Some(_) = cap.get(1) {
            if !skip {
                let lhs = cap[2].parse::<i32>().unwrap();
                let rhs = cap[3].parse::<i32>().unwrap();
                operations.push(Instruction::Mul(lhs, rhs));
            }
        } else if let Some(_) = cap.get(4) {
            skip = true;
        } else if let Some(_) = cap.get(5) {
            skip = false;
        }
    }
    operations
}

fn execute(operations: Vec<Instruction>) -> i32 {
    operations
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(lhs, rhs) => lhs * rhs,
        })
        .sum()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let source_code = load_program_source_code(file_path)?;
    println!("Program: {:?}", source_code);

    let program = compile(&source_code);
    let result = execute(program);
    println!("Program result: {}", result);

    let program = compile_with_reenabling_feature(&source_code);
    let result = execute(program);
    println!("Program result with re-enabling feature: {}", result);

    Ok(())
}
