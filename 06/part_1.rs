mod shared;

use std::collections::HashSet;

fn main() {
    let mut row_dir = -1i32;
    let mut col_dir = 0i32;
    let (lab, mut guard_row, mut guard_col) = shared::build_lab();
    let mut visited:HashSet<(usize, usize)> = HashSet::new();
    visited.insert((guard_row.clone(), guard_col.clone()));
    while !shared::check_done(&lab, guard_row, guard_col, row_dir, col_dir) {
        let (mut next_row, mut next_col) = shared::next(guard_row, guard_col, row_dir, col_dir);
        while lab[next_row][next_col] == '#' {
            shared::rotate_right(&mut row_dir, &mut col_dir);
            (next_row, next_col) = shared::next(guard_row, guard_col, row_dir, col_dir);
        }
        guard_row = next_row;
        guard_col = next_col;
        visited.insert((guard_row.clone(), guard_col.clone()));
    }
    shared::print_lab(&lab, guard_row, guard_col);
    println!("The guard is at row {} col {} ({}).", guard_row, guard_col, visited.len());
}
