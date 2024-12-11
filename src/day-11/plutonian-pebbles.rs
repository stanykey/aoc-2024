use std::io;
use std::time::Instant;

fn parse_stones(puzzle: &str) -> Vec<u64> {
    puzzle
        .split_whitespace()
        .map(|line| line.parse::<u64>().expect("not a number"))
        .collect()
}

fn blink(stones: &[u64]) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|&stone| match stone {
            0 => vec![1],
            _ if stone.to_string().len() % 2 == 0 => {
                let stone_str = stone.to_string();
                let mid = stone_str.len() / 2;
                let (first, second) = stone_str.split_at(mid);

                first
                    .parse::<u64>()
                    .ok()
                    .into_iter()
                    .chain(second.parse::<u64>().ok())
                    .collect()
            }
            _ => vec![stone * 2024],
        })
        .collect()
}

fn count_stones_after_blinking(stones: &Vec<u64>, count: usize) -> usize {
    (0..count)
        .fold(stones.to_vec(), |stones, _| blink(&stones))
        .len()
}

fn main() -> io::Result<()> {
    let puzzle_input = include_str!("input.data");
    let stones = parse_stones(puzzle_input);

    let timer = Instant::now();
    let blink_count = 25;
    let stone_count = count_stones_after_blinking(&stones, blink_count);
    println!(
        "There will be {} stones after you blink {} times",
        stone_count, blink_count
    );
    println!("Time elapsed: {:?}", timer.elapsed());

    Ok(())
}
