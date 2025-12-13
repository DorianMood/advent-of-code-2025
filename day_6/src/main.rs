use std::{fs, str::FromStr};

use num_bigint::BigInt;
use num_traits::{One, Zero};

const FILE_PATH: &str = "assets/input.txt";
// const FILE_PATH: &str = "assets/mock.txt";

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("Failed to read input file");

    let mut lines: Vec<&str> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();

    let ops: Vec<char> = lines
        .pop()
        .expect("No operations line")
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let rows = lines.len();
    let cols = ops.len();

    let mut numbers: Vec<BigInt> = Vec::with_capacity(rows * cols);

    for line in &lines {
        let values: Vec<BigInt> = line
            .split_whitespace()
            .map(|x| BigInt::from_str(x).expect("Bad number"))
            .collect();

        assert_eq!(values.len(), cols, "Wrong number of columns");

        numbers.extend(values);
    }

    let mut result: Vec<BigInt> = Vec::with_capacity(cols);

    for j in 0..cols {
        let mut acc = match ops[j] {
            '+' => BigInt::zero(),
            '*' => BigInt::one(),
            _ => panic!("Unknown operation"),
        };

        for i in 0..rows {
            let v = &numbers[i * cols + j];
            match ops[j] {
                '+' => acc += v,
                '*' => acc *= v,
                _ => unreachable!(),
            }
        }

        result.push(acc);
    }

    let problems_sum: BigInt = result.iter().cloned().sum();

    println!("2. {}", problems_sum);
}
