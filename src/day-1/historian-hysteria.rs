use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines(file_path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_locations_data(file_path: &Path) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut left_data = Vec::new();
    let mut right_data = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(entry) = line {
                let parts: Vec<&str> = entry.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(left), Ok(right)) =
                        (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                    {
                        left_data.push(left);
                        right_data.push(right);
                    }
                }
            }
        }
    }

    Ok((left_data, right_data))
}

fn get_total_distance(mut left_data: Vec<i32>, mut right_data: Vec<i32>) -> i32 {
    left_data.sort();
    right_data.sort();

    left_data
        .iter()
        .zip(right_data.iter())
        .map(|(lhs, rhs)| (lhs - rhs).abs())
        .sum()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("test.data");
    let (left_data, right_data) = read_locations_data(file_path)?;
    // println!("Left data: {:?}", left_data);
    // println!("Right data: {:?}", right_data);

    println!(
        "Total distance is {:?}",
        get_total_distance(left_data.clone(), right_data.clone())
    );

    Ok(())
}
