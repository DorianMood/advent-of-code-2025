use std::{cmp::max, fs, vec};

const FILE_PATH: &str = "assets/input.txt";
// const FILE_PATH: &str = "assets/mock.txt";

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("Failed to read input file");

    let mut total_output_joltage = 0;
    let mut max_number_from_digits_result = 0;

    for bank in input.lines() {
        total_output_joltage += find_bank_output_joltage(bank);
        max_number_from_digits_result += max_number_from_digits(bank, 12);
    }

    println!("1. {}", total_output_joltage);
    println!("2. {}", max_number_from_digits_result);
}

fn find_bank_output_joltage(bank: &str) -> u32 {
    let digits: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let size = bank.len();
    let mut max_right: Vec<u32> = vec![0; size];

    let mut biggest_so_far = *digits.last().unwrap();
    for i in (0..size).rev() {
        max_right[i] = biggest_so_far;
        biggest_so_far = max(digits[i], biggest_so_far)
    }

    let mut best: u32 = 0;
    for i in 0..size - 1 {
        let candidate = digits[i] * 10 + max_right[i];
        if candidate > best {
            best = candidate;
        }
    }

    best
}

fn max_number_from_digits(bank: &str, k: usize) -> u64 {
    let digits: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let n = digits.len();
    let mut stack: Vec<u32> = Vec::with_capacity(k);

    for i in 0..n {
        let remaining = n - i;
        while let Some(&last) = stack.last() {
            if last < digits[i] && stack.len() - 1 + remaining >= k {
                stack.pop();
            } else {
                break;
            }
        }
        if stack.len() < k {
            stack.push(digits[i]);
        }
    }

    // Превращаем стек в число
    let mut number: u64 = 0;
    for &d in &stack {
        number = number * 10 + d as u64;
    }

    number
}
