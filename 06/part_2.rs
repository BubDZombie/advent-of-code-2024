mod shared;

use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
    row_dir: i32,
    col_dir: i32,
}

fn main() {
    let (original_lab, mut original_guard_row, mut original_guard_col) = shared::build_lab();
    let mut try_count = 0;
    let mut loop_count = 0;
    for row in 0..original_lab.len() {
        for col in 0..original_lab[0].len() {
            try_count += 1;
            if original_lab[row][col] == '#'
                || (row == original_guard_row && col == original_guard_col) {
                    continue;
                }
            let mut lab = original_lab.clone();
            lab[row][col] = '#';
            let mut guard_row = original_guard_row.clone();
            let mut guard_col = original_guard_col.clone();
            let mut row_dir = -1i32;
            let mut col_dir = 0i32;
            let mut visited:HashSet<Position> = HashSet::new();
            let mut looped = false;
            while !looped && !shared::check_done(&lab, guard_row, guard_col, row_dir, col_dir) {
                let (mut next_row, mut next_col) = shared::next(guard_row, guard_col, row_dir, col_dir);
                while lab[next_row][next_col] == '#' {
                    shared::rotate_right(&mut row_dir, &mut col_dir);
                    (next_row, next_col) = shared::next(guard_row, guard_col, row_dir, col_dir);
                }
                guard_row = next_row;
                guard_col = next_col;
                let position = Position{
                    row: guard_row.clone(),
                    col: guard_col.clone(),
                    row_dir: row_dir.clone(),
                    col_dir: col_dir.clone(),
                };
                if visited.contains(&position) {
                    looped = true;
                    loop_count += 1;
                } else {
                    visited.insert(position);
                }
            }
            //shared::print_lab(&lab, guard_row, guard_col);
            if try_count % 100 == 0 {
                println!("The guard is at row {} col {} and loop_count is {}.", guard_row, guard_col, loop_count);
            }
        }
    }
    println!("Found {} ways to make the guard loop.", loop_count);
}
