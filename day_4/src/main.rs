use std::{cmp::min, fs, vec};

const FILE_PATH: &str = "assets/input.txt";
// const FILE_PATH: &str = "assets/mock.txt";

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("Failed to read input file");

    // INFO: Build a grid
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let mut grid: Vec<i64> = vec![0; rows * cols];

    for (i, row) in input.lines().enumerate() {
        for (j, char) in row.chars().enumerate() {
            grid[i * cols + j] = if char == '@' { 1 } else { 0 };
        }
    }

    let prefix_diagram = make_sat(&grid, rows, cols);
    let prefix_cols = cols + 1;

    let mut num_cells_possible: i32 = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i * cols + j] == 0 {
                continue;
            }

            // 3×3 in grid
            let r1 = i.saturating_sub(1);
            let c1 = j.saturating_sub(1);
            let r2 = (i + 1).min(rows - 1);
            let c2 = (j + 1).min(cols - 1);

            // 3x3 in SAT
            let w = prefix_cols;

            let lt = r1 * w + c1;
            let rt = r1 * w + (c2 + 1);
            let lb = (r2 + 1) * w + c1;
            let rb = (r2 + 1) * w + (c2 + 1);

            let count =
                prefix_diagram[rb] - prefix_diagram[rt] - prefix_diagram[lb] + prefix_diagram[lt];

            if count <= 4 {
                num_cells_possible += 1;
            }
        }
    }

    println!("1. {}", num_cells_possible);
    // println!("2. {}", max_number_from_digits_result);
}

/**
* INFO: Prefix diagram is a way to dial with such problems
* Aka Summed area table (SAT)
**/
fn make_sat(grid: &Vec<i64>, rows: usize, cols: usize) -> Vec<i64> {
    // Используем i64 для предотвращения переполнения
    let prefix_cols = cols + 1;
    let mut prefix_diagram: Vec<i64> = vec![0; (rows + 1) * prefix_cols];

    for i in 1..=rows {
        for j in 1..=cols {
            // 1. Получаем значение из исходной сетки, преобразованное в i64
            let grid_idx = (i - 1) * cols + (j - 1);
            let grid_val = grid[grid_idx];

            // 2. Вычисляем индексы для prefix_diagram
            // W = prefix_cols
            let current_idx = i * prefix_cols + j; // SAT(i, j)
            let above_idx = (i - 1) * prefix_cols + j; // SAT(i-1, j)
            let left_idx = i * prefix_cols + (j - 1); // SAT(i, j-1)
            let diag_idx = (i - 1) * prefix_cols + (j - 1); // SAT(i-1, j-1)

            // 3. Применяем формулу (она верна)
            prefix_diagram[current_idx] =
                grid_val + prefix_diagram[above_idx] + prefix_diagram[left_idx]
                    - prefix_diagram[diag_idx];
        }
    }

    prefix_diagram
}
