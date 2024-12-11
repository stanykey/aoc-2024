use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::Instant;

fn load_topographic_map(file_path: &Path) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let map = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|height| height.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    Ok(map)
}

fn calculate_trailhead_score(map: &Vec<Vec<u8>>, start_row: usize, start_col: usize) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col, 0));
    visited[start_row][start_col] = true;

    let mut reachable_nines = 0;

    while let Some((row, col, height)) = queue.pop_front() {
        if height == 9 {
            reachable_nines += 1;
            continue;
        }

        let valid_neighbors = [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .map(|&(dr, dc)| (row as isize + dr, col as isize + dc))
            .filter(|&(new_row, new_col)| {
                new_row >= 0
                    && new_col >= 0
                    && (new_row as usize) < map.len()
                    && (new_col as usize) < map[0].len()
            })
            .map(|(new_row, new_col)| (new_row as usize, new_col as usize))
            .filter(|&(new_row, new_col)| {
                !visited[new_row][new_col] && map[new_row][new_col] == height + 1
            })
            .collect::<Vec<(usize, usize)>>();

        for (new_row, new_col) in valid_neighbors {
            visited[new_row][new_col] = true;
            queue.push_back((new_row, new_col, height + 1));
        }
    }

    reachable_nines
}

fn calculate_total_trailhead_scores(map: &Vec<Vec<u8>>) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().map(move |(col_idx, &height)| {
                if height == 0 {
                    calculate_trailhead_score(map, row_idx, col_idx)
                } else {
                    0
                }
            })
        })
        .sum()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let topographic_map = load_topographic_map(file_path).expect("Could not load map");

    let timer = Instant::now();
    let score = calculate_total_trailhead_scores(&topographic_map);
    println!("Total score of all trailheads: {}", score);
    println!("Time elapsed: {:?}", timer.elapsed());

    Ok(())
}
