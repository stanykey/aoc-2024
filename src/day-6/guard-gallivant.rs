use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn load_lab_map(file_path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    lines.collect::<Result<Vec<_>, _>>()
}

fn find_guard(map: &[String]) -> ((usize, usize), char) {
    for (y, row) in map.iter().enumerate() {
        for (x, symbol) in row.chars().enumerate() {
            if ['^', '>', 'v', '<'].contains(&symbol) {
                return ((y, x), symbol);
            }
        }
    }
    panic!("Guard not found on the map");
}

fn turn_right(direction: char) -> char {
    match direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Invalid direction"),
    }
}

fn move_forward(position: (usize, usize), direction: char) -> (usize, usize) {
    match direction {
        '^' => (position.0.wrapping_sub(1), position.1),
        '>' => (position.0, position.1 + 1),
        'v' => (position.0 + 1, position.1),
        '<' => (position.0, position.1.wrapping_sub(1)),
        _ => panic!("Invalid direction"),
    }
}

fn is_in_bounds(position: (usize, usize), map: &[String]) -> bool {
    position.0 < map.len() && position.1 < map[0].len()
}

fn is_obstacle(position: (usize, usize), map: &[String]) -> bool {
    map[position.0].chars().nth(position.1).unwrap_or('.') == '#'
}

fn simulate_guard(map: &[String]) -> usize {
    let ((mut y, mut x), mut direction) = find_guard(map);
    let mut visited = HashSet::new();
    visited.insert((y, x));

    loop {
        let next_position = move_forward((y, x), direction);

        if !is_in_bounds(next_position, map) {
            break;
        }

        if is_obstacle(next_position, map) {
            direction = turn_right(direction);
        } else {
            (y, x) = next_position;
            visited.insert((y, x));
        }
    }

    visited.len()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let lab_map = load_lab_map(file_path)?;

    let distinct_positions = simulate_guard(&lab_map);
    println!("Distinct positions visited: {}", distinct_positions);

    Ok(())
}
