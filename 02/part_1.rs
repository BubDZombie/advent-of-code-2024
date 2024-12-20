use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut num_safe = 0;

    for line in reader.lines() {
        let mut report: Vec<i32> = Vec::new();
        for num in line.unwrap().split_whitespace() {
            report.push(num.parse().unwrap());
        }
        let mut ascdesc = "asc";
        if report[1] < report[0] {
            ascdesc = "desc";
        }
        let mut i = 0;
        let mut safe = true;
        while safe && i < report.len() - 1 {
            if report[i] == report[i+1]
                       || ascdesc == "asc" && report[i+1] < report[i]
                       || ascdesc == "desc" && report[i+1] > report[i]
                       || (report[i] - report[i+1]).abs() > 3 {
                safe = false;
            }
            i += 1
        }
        if safe {
            num_safe += 1;
        }
        println!("{:?} {} {} {}", report, ascdesc, safe, num_safe);
    }

    Ok(())
}
