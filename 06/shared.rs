use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::thread;
use std::time;

// Opens input.txt, returns a matrix of chars representing the obstacles,
// and the guard's row and column as usize.
pub fn build_lab() -> (Vec<Vec<char>>, usize, usize) {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lab:Vec<Vec<char>> = Vec::new();
    let mut guard_row = 0;
    let mut guard_col = 0;
    for (row_num, row) in reader.lines().enumerate() {
        let mut lab_row:Vec<char> = Vec::new();
        for (col_num, c) in row.unwrap().chars().enumerate() {
            if c == '#' {
                lab_row.push('#');
            } else {
                lab_row.push(' ');
                if c == '^' {
                    guard_row = row_num;
                    guard_col = col_num;
                }
            }
        }
        lab.push(lab_row);
    }
    return (lab, guard_row, guard_col);
}

pub fn next(guard_row:usize, guard_col:usize, row_dir:i32, col_dir:i32) -> (usize, usize){
    let mut next_row = guard_row;
    let mut next_col = guard_col;
    if row_dir < 0 {
        next_row = guard_row - 1;
    } else if row_dir > 0 {
        next_row = guard_row + 1;
    } else if col_dir < 0 {
        next_col = guard_col -1 ;
    } else if col_dir > 0 {
        next_col = guard_col + 1;
    }
    return (next_row, next_col);
}

pub fn check_done(
    lab:&Vec<Vec<char>>,
    guard_row:usize,
    guard_col:usize,
    row_dir:i32,
    col_dir:i32) -> bool
{
    if (guard_row == 0 && row_dir == -1)
        || (guard_col == 0 && col_dir == -1)
        || (guard_row == lab.len() - 1 && row_dir == 1)
        || (guard_col == lab[0].len() - 1 && col_dir == 1) {
            return true;
        }
    return false;
}

pub fn print_lab(lab:&Vec<Vec<char>>, guard_row:usize, guard_col:usize) {
    for (row_num, row) in lab.iter().enumerate() {
        for (col_num, c) in row.iter().enumerate() {
            if row_num == guard_row && col_num == guard_col {
                print!("O");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    thread::sleep(time::Duration::from_millis(50));
}

pub fn rotate_right(row_dir:&mut i32, col_dir:&mut i32) {
    if *row_dir == -1 {
        *row_dir = 0;
        *col_dir = 1;
    } else if *col_dir == 1 {
        *col_dir = 0;
        *row_dir = 1;
    } else if *row_dir == 1 {
        *row_dir = 0;
        *col_dir = -1;
    } else {
        *row_dir = -1;
        *col_dir = 0;
    }
}
