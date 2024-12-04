use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn load_word_search(file_path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    lines.collect::<Result<Vec<_>, _>>() // Collect and propagate errors
}

fn check_word(
    word_search: &Vec<String>,
    word: &str,
    row: isize,
    col: isize,
    dir: (isize, isize),
) -> bool {
    let rows = word_search.len() as isize;
    let cols = word_search[0].len() as isize;
    let word_len = word.len();

    for i in 0..word_len {
        let new_row = row + i as isize * dir.0;
        let new_col = col + i as isize * dir.1;
        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }

        let lhs = word_search[new_row as usize].chars().nth(new_col as usize);
        let rhs = Some(word.chars().nth(i).unwrap());
        if lhs != rhs {
            return false;
        }
    }
    true
}

fn count_word(word_search: &Vec<String>, word: &str) -> usize {
    let mut count = 0;

    let directions = vec![
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (0, -1),  // Left (reverse horizontal)
        (-1, 0),  // Up (reverse vertical)
        (-1, -1), // Diagonal up-left
        (-1, 1),  // Diagonal up-right
    ];

    // Iterate over every cell in the grid
    let rows = word_search.len();
    let cols = word_search[0].len();
    for row in 0..rows {
        for col in 0..cols {
            for &dir in &directions {
                if check_word(&word_search, word, row as isize, col as isize, dir) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_xmas_patterns(word_search: &Vec<String>) -> usize {
    let mut count = 0;

    let rows = word_search.len();
    let cols = word_search[0].len();
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if word_search[row].chars().nth(col).unwrap() == 'A' {
                let prev_row = row as isize - 1;
                let prev_col = col as isize - 1;
                let next_col = col as isize + 1;

                let top_left_to_bottom_right =
                    check_word(word_search, "MAS", prev_row, prev_col, (1, 1))
                        || check_word(word_search, "SAM", prev_row, prev_col, (1, 1));

                let top_right_to_bottom_left =
                    check_word(word_search, "MAS", prev_row, next_col, (1, -1))
                        || check_word(word_search, "SAM", prev_row, next_col, (1, -1));

                if top_left_to_bottom_right && top_right_to_bottom_left {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() -> io::Result<()> {
    let file_path = Path::new("input.data");
    let word_search = load_word_search(file_path)?;
    // println!("{:?}", word_search);

    let word = "XMAS";
    let word_count = count_word(&word_search, word);
    println!("The {} appears {} times", word, word_count);

    let xmas_pattern_count = count_xmas_patterns(&word_search);
    println!("The 'x-max' appears {} times", xmas_pattern_count);

    Ok(())
}
