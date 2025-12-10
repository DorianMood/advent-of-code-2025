use std::cmp::max;
use std::fs;

fn main() {
    let input = fs::read_to_string("assets/input.txt").expect("Failed to read input file");

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

        if rotation >= 100 {
            zeros_in_all_rotations += rotation / 100;
        }

        match first {
            'L' => {
                if position - rotation < 0 {
                    zeros_in_all_rotations += 1;
                }
            }
            'R' => {
                if position + rotation >= 100 {
                    zeros_in_all_rotations += 1;
                }
            }
            _ => {}
        }

        position = (position + direction * rotation).rem_euclid(100);

        if position == 0 {
            zeros_count += 1;
            zeros_in_all_rotations += 1;
        }
    }

    println!("{}", zeros_count);
    println!("{}", zeros_in_all_rotations);
}
