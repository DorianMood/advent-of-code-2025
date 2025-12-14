use std::{collections::HashSet, fs};

const FILE_PATH: &str = "assets/input.txt";
// const FILE_PATH: &str = "assets/mock.txt";

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("Failed to read input file");

    let mut lines = input.lines();

    let first_row = lines.next().unwrap();
    let cols = first_row.len();

    let start_index = first_row
        .chars()
        .into_iter()
        .position(|c| c == 'S')
        .expect("Стартовый символ 'S' не найден");

    let start_col = start_index % cols;

    let mut active_positions: HashSet<usize> = HashSet::new();
    active_positions.insert(start_col);

    let mut visited_columns: HashSet<usize> = HashSet::new();
    visited_columns.insert(start_col);

    let mut counter = 0;

    while let Some(row) = lines.next() {
        if active_positions.is_empty() {
            break;
        }
        let mut next_active_positions: HashSet<usize> = HashSet::new();

        let row_chars: Vec<char> = row.chars().collect();

        for &col in active_positions.iter() {
            let character = row_chars[col];

            match character {
                '.' => {
                    next_active_positions.insert(col);
                }
                '^' => {
                    counter += 1;
                    if col > 0 {
                        next_active_positions.insert(col - 1);
                    }
                    if col < cols - 1 {
                        next_active_positions.insert(col + 1);
                    }
                }
                _ => {}
            }
        }

        for &col in next_active_positions.iter() {
            visited_columns.insert(col);
        }

        active_positions = next_active_positions;
    }

    println!("1. {counter}");
}
