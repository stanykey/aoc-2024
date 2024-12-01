use std::collections::HashMap;
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
                if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    left_data.push(left);
                    right_data.push(right);
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

fn get_similarity_score(left_data: Vec<i32>, right_data: Vec<i32>) -> i32 {
    let mut freq_map = HashMap::new();
    for num in right_data {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    left_data
        .iter()
        .map(|num| freq_map.get(num).unwrap_or(&0) * num)
        .sum()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let (left_data, right_data) = read_locations_data(file_path)?;
    // println!("Left data: {:?}", left_data);
    // println!("Right data: {:?}", right_data);

    let total_distance = get_total_distance(left_data.clone(), right_data.clone());
    println!("Total distance is {:?}", total_distance);

    let similarity_score = get_similarity_score(left_data, right_data);
    println!("Similarity score is {:?}", similarity_score);

    Ok(())
}
