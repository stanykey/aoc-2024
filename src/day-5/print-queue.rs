use std::collections::HashSet;
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

fn sum_middle_pages(updates: Vec<Vec<i32>>, rules: Vec<(i32, i32)>) -> i32 {
    updates
        .iter()
        .filter(|update| is_valid_update(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let (rules, updates) = load_printer_instructions(file_path)?;
    // println!("{:?}", rules);
    // println!("{:?}", updates);

    let middle_pages_sum = sum_middle_pages(updates, rules);
    println!("The sum of middle pages numbers is {}", middle_pages_sum);

    Ok(())
}
