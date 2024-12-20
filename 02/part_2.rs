use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut num_safe = 0;

    for line in reader.lines() {
        let mut report: Vec<i8> = Vec::new();
        for num in line.unwrap().split_whitespace() {
            report.push(num.parse().unwrap());
        }
        if check_report(&report) {
            num_safe += 1;
        } else {
            let mut to_remove = 0;
            let mut fixed = false;
            while !fixed && to_remove < report.len() {
                let modified = modify_report(&report, to_remove as i8);
                if check_report(&modified) {
                    num_safe += 1;
                    fixed = true;
                }
                to_remove += 1;
            }
        }

        println!("{:?} {}", report, num_safe);
    }

    Ok(())
}

/// Return true for no errors, else false.
fn check_report(report: &Vec<i8>) -> bool {
    let mut ascdesc = "asc";
    if report[1] < report[0] {
        ascdesc = "desc";
    }
    let mut i = 0;
    while i < report.len() - 1 {
        if report[i] == report[i+1]
            || ascdesc == "asc" && report[i+1] < report[i]
            || ascdesc == "desc" && report[i+1] > report[i]
            || (report[i] - report[i+1]).abs() > 3 {
                return false
            }
        i += 1;
    }
    return true
}

/// Return a copy of the report with the element at index removed.
fn modify_report(report: &Vec<i8>, index: i8) -> Vec<i8> {
    let mut new_report: Vec<i8> = Vec::new();
    let mut i = 0;
    for num in report {
        if i != index {
            new_report.push(*num);
        }
        i += 1;
    }
    return new_report
}
