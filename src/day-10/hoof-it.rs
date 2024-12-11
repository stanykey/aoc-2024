use std::collections::VecDeque;
use std::io;
use std::time::Instant;

fn parse_topographic_map(puzzle: &str) -> Vec<Vec<u8>> {
    let map = puzzle
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    map
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

fn calculate_trailhead_rating(map: &Vec<Vec<u8>>, start_row: usize, start_col: usize) -> usize {
    let mut distinct_trails = 0;

    fn dfs(
        map: &Vec<Vec<u8>>,
        row: usize,
        col: usize,
        current_height: u8,
        visited: &mut Vec<Vec<bool>>,
        trails_count: &mut usize,
    ) {
        if current_height == 9 {
            *trails_count += 1;
            return;
        }

        visited[row][col] = true;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for &(dr, dc) in &directions {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0
                && new_col >= 0
                && (new_row as usize) < map.len()
                && (new_col as usize) < map[0].len()
            {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if !visited[new_row][new_col] && map[new_row][new_col] == current_height + 1 {
                    dfs(
                        map,
                        new_row,
                        new_col,
                        current_height + 1,
                        visited,
                        trails_count,
                    );
                }
            }
        }

        visited[row][col] = false; // Unmark the cell for other paths
    }

    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    dfs(
        map,
        start_row,
        start_col,
        0,
        &mut visited,
        &mut distinct_trails,
    );

    distinct_trails
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

fn calculate_total_trailhead_ratings(map: &Vec<Vec<u8>>) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().map(move |(col_idx, &height)| {
                if height == 0 {
                    calculate_trailhead_rating(map, row_idx, col_idx)
                } else {
                    0
                }
            })
        })
        .sum()
}

fn main() -> io::Result<()> {
    let puzzle_input = include_str!("input.data");
    let topographic_map = parse_topographic_map(puzzle_input);

    let timer = Instant::now();
    let score = calculate_total_trailhead_scores(&topographic_map);
    println!("Total score of all trailheads: {}", score);
    println!("Time elapsed: {:?}", timer.elapsed());

    let timer = Instant::now();
    let rating = calculate_total_trailhead_ratings(&topographic_map);
    println!("Total rating of all trailheads: {}", rating);
    println!("Time elapsed: {:?}", timer.elapsed());

    Ok(())
}
