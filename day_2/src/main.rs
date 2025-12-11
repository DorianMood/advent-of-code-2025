use std::fs;

// const FILE_PATH: &str = "assets/input.txt";
const FILE_PATH: &str = "assets/mock.txt";

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("Failed to read input file");

    let mut invalid_ids_sum: u64 = 0;
    let mut new_invalid_ids_sum: u64 = 0;

    for range in input.trim().split(",") {
        let mut pair = range.split("-");
        let start: u64 = pair.next().expect("Missing start").parse().unwrap();
        let end: u64 = pair.next().expect("Missing end").parse().unwrap();

        for i in start..end {
            if is_invalid_id_simple(i) {
                invalid_ids_sum += i;
            }
            if is_invalid_id(i) {
                new_invalid_ids_sum += i;
            }
        }
    }

    println!("1. {}", invalid_ids_sum);
    println!("2. {}", new_invalid_ids_sum);
}

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();

    for subset_size in 1..=s.len() / 2 {
        if !s.len().is_multiple_of(subset_size) {
            continue;
        }

        let base = &s[..subset_size];

        let repeats = s.len() / subset_size;

        if (1..repeats).all(|i| &s[i * subset_size..(i + 1) * subset_size] == base) {
            return true;
        }
    }

    false
}

fn is_invalid_id_simple(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let half = len / 2;

    s[..half] == s[half..]
}
