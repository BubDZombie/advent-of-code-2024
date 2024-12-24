use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub struct Equation {
    pub test_value: usize,
    pub operands: Vec<usize>,
}

pub fn parse_input() -> Vec<Equation> {
    let mut equations:Vec<Equation> = Vec::new();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        let parts:Vec<&str> = l.split(':').collect();
        let test_value:usize = parts[0].parse::<usize>().unwrap();
        let mut operands:Vec<usize> = Vec::new();
        for operand_string in parts[1].split_whitespace() {
            operands.push(operand_string.parse::<usize>().unwrap());
        }
        equations.push(Equation{test_value:test_value, operands:operands});
    }
    return equations;
}
