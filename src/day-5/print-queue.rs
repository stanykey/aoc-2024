use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn load_printer_instructions(file_path: &Path) -> io::Result<(Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let file = File::open(file_path)?;
    let mut lines = io::BufReader::new(file).lines().filter_map(Result::ok);

    // Collect the rules section until the first empty line
    let rules: Vec<(i32, i32)> = lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            (
                parts[0].trim().parse::<i32>().unwrap(),
                parts[1].trim().parse::<i32>().unwrap(),
            )
        })
        .collect();

    // Process the remaining lines as updates
    let updates: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split(',')
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    Ok((rules, updates))
}

fn is_valid_update(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let page_set: HashSet<i32> = update.iter().cloned().collect();

    for &(lhs, rhs) in rules {
        if page_set.contains(&lhs) && page_set.contains(&rhs) {
            let lhs_position = update.iter().position(|&page| page == lhs).unwrap();
            let rhs_position = update.iter().position(|&page| page == rhs).unwrap();
            if lhs_position >= rhs_position {
                return false;
            }
        }
    }
    true
}

fn sum_middle_pages(updates: &Vec<Vec<i32>>, rules: &Vec<(i32, i32)>) -> i32 {
    updates
        .iter()
        .filter(|update| is_valid_update(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[allow(dead_code)]
fn correct_update_with_brute_force(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut corrected_update = update.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for &(lhs, rhs) in rules {
            let lhs_position = corrected_update.iter().position(|&page| page == lhs);
            let rhs_position = corrected_update.iter().position(|&page| page == rhs);

            if let (Some(lhs_idx), Some(rhs_idx)) = (lhs_position, rhs_position) {
                if lhs_idx > rhs_idx {
                    corrected_update.swap(lhs_idx, rhs_idx);
                    changed = true;
                }
            }
        }
    }

    corrected_update
}

fn correct_update_with_topological_sort(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut in_degree = HashMap::new();
    let mut graph = HashMap::new();

    // Initialize graph and in-degree count
    for &page in update {
        in_degree.insert(page, 0);
        graph.insert(page, vec![]);
    }

    // Build graph and in-degree based on rules
    for &(lhs, rhs) in rules {
        if update.contains(&lhs) && update.contains(&rhs) {
            graph.get_mut(&lhs).unwrap().push(rhs);
            *in_degree.get_mut(&rhs).unwrap() += 1;
        }
    }

    // Perform topological sorting
    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(&page, _)| page)
        .collect();

    let mut sorted = Vec::new();
    while let Some(page) = queue.pop_front() {
        sorted.push(page);
        for &neighbor in &graph[&page] {
            let degree = in_degree.get_mut(&neighbor).unwrap();
            *degree -= 1;
            if *degree == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    sorted
}

fn sum_middle_pages_with_corrections(updates: &Vec<Vec<i32>>, rules: &Vec<(i32, i32)>) -> i32 {
    updates
        .iter()
        .filter(|update| !is_valid_update(update, rules))
        .map(|update| {
            let corrected = correct_update_with_topological_sort(update, rules);
            corrected[corrected.len() / 2]
        })
        .sum()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("test.data");
    let (rules, updates) = load_printer_instructions(file_path)?;
    // println!("{:?}", rules);
    // println!("{:?}", updates);

    let middle_pages_sum = sum_middle_pages(&updates, &rules);
    println!("The sum of middle pages numbers is {}", middle_pages_sum);

    let middle_pages_sum = sum_middle_pages_with_corrections(&updates, &rules);
    println!("The sum of middle pages numbers is {}", middle_pages_sum);

    Ok(())
}
