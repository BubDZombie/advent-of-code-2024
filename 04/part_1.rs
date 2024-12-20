use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut search_strings: Vec<String> = Vec::new();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        search_strings.push(l.clone());
        let v: Vec<char> = l.chars().collect();
        search_strings.push(v.iter().rev().collect::<String>());
        matrix.push(v);
    }

    for col in 0..matrix[0].len() {
        let mut v: Vec<char> = Vec::new();
        for row in matrix.iter() {
            v.push(row[col]);
        }
        search_strings.push(v.clone().iter().rev().collect::<String>());
        search_strings.push(v.iter().collect::<String>());
    }

    // Right-handed diagonals.
    let mut starting_points: Vec<(usize, usize)> = Vec::new();
    for row_start in 0..matrix.len() - 3 {
        starting_points.push((row_start, 0));
    }
    for col_start in 1..matrix[0].len() - 3 {
        starting_points.push((0, col_start));
    }
    for (row_start, col_start) in starting_points {
        let mut row = row_start;
        let mut col = col_start;
        let mut v: Vec<char> = Vec::new();
        while row < matrix.len() && col < matrix[1].len() {
            v.push(matrix[row][col]);
            row += 1;
            col += 1;
        }
        search_strings.push(v.clone().iter().rev().collect::<String>());
        search_strings.push(v.iter().collect::<String>());
    }

    // Left-handed diagonals.
    let mut starting_points: Vec<(usize, usize)> = Vec::new();
    for row_start in 0..matrix.len() - 3 {
        starting_points.push((row_start, matrix[0].len() - 1));
    }
    for col_start in 3..matrix[0].len() - 1{
        starting_points.push((0, col_start));
    }
    for (row_start, col_start) in starting_points {
        let mut row = row_start;
        let mut col = col_start as i32;
        let mut v: Vec<char> = Vec::new();
        while row < matrix.len() && col >= 0 {
            v.push(matrix[row][col as usize]);
            row += 1;
            col -= 1;
        }
        search_strings.push(v.clone().iter().rev().collect::<String>());
        search_strings.push(v.iter().collect::<String>());
    }

    let xmas = Regex::new(r"XMAS").unwrap();
    let mut xmas_count = 0;
    for search in &search_strings {
        //println!("{:?}", search);
        for _ in xmas.find_iter(&search) {
            xmas_count += 1;
        }
    }
    println!("The string XMAS was found {} times.", xmas_count);

    // Look for Xs composed of MAS
    let mut x_count = 0;
    for row in 1..matrix.len() - 1 {
        for col in 1..matrix[0].len() - 1 {
            if matrix[row][col] == 'A' {
                let arm1 = vec![matrix[row-1][col-1], matrix[row+1][col+1]];
                let arm2 = vec![matrix[row-1][col+1], matrix[row+1][col-1]];
                if arm1.contains(&'M')
                    && arm1.contains(&'S')
                    && arm2.contains(&'M')
                    && arm2.contains(&'S') {
                        x_count += 1;
                    }
            }
        }
    }
    println!("Xs of MAS were found {} times.", x_count);

    Ok(())
}
