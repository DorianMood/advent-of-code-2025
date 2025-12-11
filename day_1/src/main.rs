use std::fs;

// const FILE_PATH = "assets/input.txt"
const FILE_PATH: &str = "assets/mock.txt";

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("Failed to read input file");

    let mut position: i32 = 50;
    let mut zeros_count: i32 = 0;
    let mut zeros_in_all_rotations: i32 = 0;

    for line in input.lines() {
        let first = line.chars().next().unwrap();
        let direction = match first {
            'R' => 1,
            'L' => -1,
            _ => 0,
        };

        let rotation: i32 = line[1..].parse().unwrap();

        let mut zeros_passed = 0;

        match first {
            'L' => {
                let curr_pos = position - rotation;
                if curr_pos < 0 {
                    zeros_passed = curr_pos.div_euclid(100).abs();
                    if position == 0 {
                        zeros_passed -= 1;
                    }
                }
            }
            'R' => {
                let curr_pos = position + rotation;
                if curr_pos > 100 {
                    zeros_passed = curr_pos / 100;
                    if curr_pos.rem_euclid(100) == 0 {
                        zeros_passed -= 1;
                    }
                }
            }
            _ => {}
        }

        position = (position + direction * rotation).rem_euclid(100);

        if position == 0 {
            zeros_count += 1;
            zeros_passed += 1;
        }

        zeros_in_all_rotations += zeros_passed;
    }

    println!("1. {}", zeros_count);
    println!("2. {}", zeros_in_all_rotations);
}
