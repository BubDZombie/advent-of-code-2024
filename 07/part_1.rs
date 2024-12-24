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
                } else {
                    sum *= equation.operands[i + 1];
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
    for i in 0..i32::pow(2, count as u32) {
        let mut shifter = i.clone();
        let mut operators:Vec<char> = Vec::new();
        for _ in 0..count {
            if shifter & 1 == 1 {
                operators.push('+');
            } else {
                operators.push('*');
            }
            shifter = shifter>>1;
        }
        options.push(operators);
    }
    return options;
}
