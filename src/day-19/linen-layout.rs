use std::collections::HashMap;

fn load_towel_patterns(input: &str) -> Vec<&str> {
    let mut patterns: Vec<&str> = input
        .lines()
        .next()
        .expect("Empty input")
        .split(", ")
        .collect();
    patterns.sort_by(|lhs, rhs| lhs.len().cmp(&rhs.len()).reverse());

    patterns
}

fn load_towel_designs(input: &str) -> Vec<&str> {
    input.lines().skip(2).collect()
}

fn find_combination(design: &str, patterns: &Vec<&str>, memo: &mut HashMap<String, bool>) -> bool {
    if let Some(&result) = memo.get(design) {
        return result;
    }

    if design.is_empty() {
        memo.insert(design.to_string(), true);
        return true;
    }

    for pattern in patterns.iter() {
        if design.starts_with(pattern) {
            let remaining_design = &design[pattern.len()..];
            if find_combination(remaining_design, patterns, memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }
    }

    memo.insert(design.to_string(), false);
    false
}

fn count_possible_combination(
    design: &str,
    patterns: &Vec<&str>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&result) = memo.get(design) {
        return result;
    }

    if design.is_empty() {
        memo.insert(design.to_string(), 1);
        return 1;
    }

    let mut total_ways = 0;
    for pattern in patterns.iter() {
        if design.starts_with(pattern) {
            let remaining_design = &design[pattern.len()..];
            total_ways += count_possible_combination(remaining_design, patterns, memo);
        }
    }

    memo.insert(design.to_string(), total_ways);
    total_ways
}

fn count_valid_designs(patterns: &Vec<&str>, designs: &Vec<&str>) -> usize {
    let mut memo = HashMap::new();
    designs
        .iter()
        .filter(|design| find_combination(design, &patterns, &mut memo))
        .count()
}

fn count_all_possible_combination(patterns: &Vec<&str>, designs: &Vec<&str>) -> usize {
    let mut memo = HashMap::new();
    designs
        .iter()
        .map(|design| count_possible_combination(design, &patterns, &mut memo))
        .sum()
}

fn main() {
    let input = include_str!("input.data");
    let patterns = load_towel_patterns(input);
    let designs = load_towel_designs(input);

    let design_count = count_valid_designs(&patterns, &designs);
    println!("There are {} valid designs", design_count);

    let possible_design_count = count_all_possible_combination(&patterns, &designs);
    println!("There are {} possible designs", possible_design_count);
}
