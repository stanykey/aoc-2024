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

fn analyze_report(report: &Vec<i32>) -> bool {
    // Check if the report is monotonic (either all increasing or all decreasing)
    let is_increasing = report.windows(2).all(|slice| slice[0] <= slice[1]);
    let is_decreasing = report.windows(2).all(|slice| slice[0] >= slice[1]);

    // Check if differences between adjacent levels are within [1, 3]
    let valid_differences = report
        .windows(2)
        .all(|slice| (slice[1] - slice[0]).abs() >= 1 && (slice[1] - slice[0]).abs() <= 3);

    (is_increasing || is_decreasing) && valid_differences
}

fn get_report_statuses(reports: &Vec<Vec<i32>>) -> Vec<(usize, bool)> {
    reports
        .iter()
        .enumerate()
        .map(|(index, report)| (index as usize, analyze_report(report)))
        .collect()
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");

    let reports = read_reports(file_path)?;
    // println!("Reports: {:?}", reports);

    let statuses = get_report_statuses(&reports);
    let safe_reports_count = statuses.iter().filter(|(_, status)| *status).count();

    println!("There are {} safe reports", safe_reports_count);

    Ok(())
}
