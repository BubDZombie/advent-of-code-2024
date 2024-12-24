mod shared;

fn main() {
    let equations = shared::parse_input();

    let mut total = 0;
    for equation in equations {
        let options = get_operators(equation.operands.len() - 1);
        for operators in options {
            let mut i = 0;
            let mut sum = equation.operands[0];
            while i < operators.len() && sum <= equation.test_value {
                if operators[i] == '+' {
                    sum += equation.operands[i + 1];
                } else if operators[i] == '*' {
                    sum *= equation.operands[i + 1];
                } else {
                    let mut s = sum.to_string();
                    s.push_str(&equation.operands[i + 1].to_string());
                    sum = s.parse::<usize>().unwrap();
                }
                i += 1;
            }
            if sum == equation.test_value {
                total += sum;
                println!("{:?} {:?} {}", equation, operators, total);
                break;
            }
        }
    }
}

fn get_operators(count:usize) -> Vec<Vec<char>> {
    let mut options:Vec<Vec<char>> = Vec::new();
    for i in 0..i64::pow(3, count as u32) {
        let mut shifter = i.clone();
        let mut operators:Vec<char> = Vec::new();
        for _ in 0..count {
            let remainder = shifter % 3;
            if remainder == 0 {
                operators.push('+');
            } else if remainder == 1 {
                operators.push('*');
            } else {
                operators.push('|');
            }
            shifter = shifter / 3;
        }
        options.push(operators);
    }
    return options;
}
