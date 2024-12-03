use regex::Regex;
use std::path::Path;
use std::{fs, io};

struct Multiply {
    lhs: i32,
    rhs: i32,
}

impl Multiply {
    fn new(lhs: i32, rhs: i32) -> Self {
        Self { lhs, rhs }
    }

    fn multiply(&self) -> i32 {
        self.lhs * self.rhs
    }
}

fn load_program_source_code(file_path: &Path) -> io::Result<String> {
    let result = fs::read_to_string(file_path);
    match result {
        Ok(text) => Ok(text.trim().to_string()),
        Err(e) => Err(e),
    }
}

fn find_valid_operations(source_code: &str) -> Vec<Multiply> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut operations: Vec<Multiply> = Vec::new();
    for cap in regex.captures_iter(source_code) {
        if let (Ok(lhs), Ok(rhs)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
            operations.push(Multiply::new(lhs, rhs));
        }
    }
    operations
}

fn execute(operations: Vec<Multiply>) -> i32 {
    operations.iter().map(Multiply::multiply).sum()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let source_code = load_program_source_code(file_path)?;
    println!("Program: {:?}", source_code);

    let operations = find_valid_operations(&source_code);
    let result = execute(operations);
    println!("Result: {}", result);

    Ok(())
}
