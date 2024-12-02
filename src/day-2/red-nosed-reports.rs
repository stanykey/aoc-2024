use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_reports(file_path: &Path) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file).lines();

    let mut reports = Vec::new();
    for line in reader {
        if let Ok(entry) = line {
            let report: Vec<i32> = entry
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            reports.push(report);
        }
    }

    Ok(reports)
}

fn is_valid_report(report: &Vec<i32>) -> bool {
    // Check if the report is monotonic (either all increasing or all decreasing)
    let is_increasing = report.windows(2).all(|slice| slice[0] <= slice[1]);
    let is_decreasing = report.windows(2).all(|slice| slice[0] >= slice[1]);

    // Check if differences between adjacent levels are within [1, 3]
    let valid_differences = report
        .windows(2)
        .all(|slice| (slice[1] - slice[0]).abs() >= 1 && (slice[1] - slice[0]).abs() <= 3);

    (is_increasing || is_decreasing) && valid_differences
}

fn can_make_valid(report: &Vec<i32>, ignore_count: usize) -> bool {
    // Base case: if no items are ignored, check the report directly
    if ignore_count == 0 {
        return is_valid_report(report);
    }

    // Try ignoring up to `ignore_count` items and check if valid
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i); // Remove the current item

        if can_make_valid(&modified_report, ignore_count - 1) {
            return true;
        }
    }

    false
}

fn analyze_report(report: &Vec<i32>, max_ignore: usize) -> bool {
    for ignore_count in 0..=max_ignore {
        if can_make_valid(report, ignore_count) {
            return true;
        }
    }

    false
}

fn get_report_statuses(reports: &Vec<Vec<i32>>, max_ignore: usize) -> Vec<(usize, bool)> {
    reports
        .iter()
        .enumerate()
        .map(|(index, report)| (index, analyze_report(report, max_ignore)))
        .collect()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");

    let reports = read_reports(file_path)?;
    // println!("Reports: {:?}", reports);

    let statuses = get_report_statuses(&reports, 1);
    let safe_reports_count = statuses.iter().filter(|(_, status)| *status).count();

    println!("There are {} safe reports", safe_reports_count);

    Ok(())
}
