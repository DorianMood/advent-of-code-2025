use std::{fs, vec};

const FILE_PATH: &str = "assets/input.txt";
// const FILE_PATH: &str = "assets/mock.txt";

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("Failed to read input file");

    let paragraphs: Vec<&str> = input.split("\n\n").filter(|s| !s.is_empty()).collect();

    if paragraphs.len() != 2 {
        panic!("Wrong input format");
    }

    let mut ranges = vec![(0, 0); paragraphs[0].lines().count()];
    let mut ids: Vec<i64> = vec![0; paragraphs[1].lines().count()];

    // Extract ranges
    for (i, range) in paragraphs[0].lines().enumerate() {
        let mut iter = range.split("-");
        let left: i64 = iter.next().unwrap().parse().unwrap();
        let right: i64 = iter.next().unwrap().parse().unwrap();
        ranges[i] = (left, right);
    }

    // Extract ids
    for (i, id) in paragraphs[1].lines().enumerate() {
        ids[i] = id.parse().unwrap();
    }

    let mut fresh_counter = 0;

    for id in ids {
        for &(left, right) in &ranges {
            if left <= id && id <= right {
                fresh_counter += 1;
                break;
            }
        }
    }

    println!("1. {}", fresh_counter);

    ranges.sort_by_key(|(left, _)| *left);
    let mut total_in_ranges = 0;

    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for &(left, right) in &ranges {
        if left <= current_end {
            current_end = current_end.max(right);
        } else {
            total_in_ranges += current_end - current_start + 1;
            current_start = left;
            current_end = right;
        }
    }

    total_in_ranges += (current_end - current_start + 1) as i64;

    println!("2. {}", total_in_ranges);
}
