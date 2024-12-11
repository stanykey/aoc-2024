use std::collections::HashMap;

fn parse_stones(puzzle: &str) -> HashMap<u64, usize> {
    puzzle
        .split_whitespace()
        .map(|line| line.parse::<u64>().expect("not a number"))
        .fold(HashMap::new(), |mut map, stone| {
            *map.entry(stone).or_insert(0) += 1;
            map
        })
}

fn blink(stone_counts: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_counts = HashMap::new();

    for (&stone, &count) in stone_counts.iter() {
        match stone {
            0 => {
                *new_counts.entry(1).or_insert(0) += count;
            }
            _ if stone.to_string().len() % 2 == 0 => {
                let stone_str = stone.to_string();
                let mid = stone_str.len() / 2;
                let (first, second) = stone_str.split_at(mid);
                let left: u64 = first.parse().expect("invalid left part");
                let right: u64 = second.parse().expect("invalid right part");
                *new_counts.entry(left).or_insert(0) += count;
                *new_counts.entry(right).or_insert(0) += count;
            }
            _ => {
                let new_stone = stone * 2024;
                *new_counts.entry(new_stone).or_insert(0) += count;
            }
        }
    }

    new_counts
}

fn count_stones_after_blinking(stones: &HashMap<u64, usize>, blinks: usize) -> usize {
    let mut stone_counts = stones.clone();
    for _ in 0..blinks {
        stone_counts = blink(&stone_counts);
    }
    stone_counts.values().sum()
}

fn main() {
    let puzzle_input = include_str!("input.data");
    let stones = parse_stones(puzzle_input);

    for blink_count in [25, 75] {
        let total_stones = count_stones_after_blinking(&stones, blink_count);
        println!(
            "There will be {} stones after you blink {} times",
            total_stones, blink_count
        );
    }
}
