// https://www.codewars.com/kata/577e9095d648a15b800000d4

use regex::Regex;

fn main() {
    // Simple addition
    assert_eq!(postfix_evaluator("2 3 +"), 5);

    // Addition with negative numbers
    assert_eq!(postfix_evaluator("2 -3 +"), -1);

    // Constant numbers
    assert_eq!(postfix_evaluator("1"), 1);
    assert_eq!(postfix_evaluator("-1"), -1);

    // Complex expressions
    assert_eq!(postfix_evaluator("2 3 9 4 / + *"), 10);
    assert_eq!(postfix_evaluator("3 4 9 / *"), 0);
    assert_eq!(postfix_evaluator("4 8 + 6 5 - * 3 2 - 2 2 + * /"), 3);

    // Multi-digit
    assert_eq!(postfix_evaluator("21 21 +"), 42);
}

fn postfix_evaluator(expr: &str) -> i64 {
    let rgx = Regex::new(r"(-?[0-9]+|[-+*/])").unwrap();
    let tokens = rgx.find_iter(expr).map(|m| m.as_str()).collect::<Vec<_>>();
    // let tokens = expr.split_whitespace().collect::<Vec<_>>();  // easier

    let mut stack = Vec::new();
    for token in tokens {
        match token {
            "+" | "-" | "*" | "/" => {
                let rhs = stack.pop();
                let lhs = stack.pop();
                match (lhs, rhs) {
                    (Some(x1), Some(x2)) => match token {
                        "+" => stack.push(x1 + x2),
                        "-" => stack.push(x1 - x2),
                        "*" => stack.push(x1 * x2),
                        "/" => stack.push(x1 / x2),
                        _ => unreachable!(),
                    },
                    _ => panic!("Invalid operands for binary op"),
                }
            }
            token => match token.parse::<i64>() {
                Ok(x) => stack.push(x),
                Err(_) => panic!("Unidentified token"),
            },
        }
    }

    if stack.len() != 1 {
        panic!("Can't reduce expression");
    }

    *stack.first().unwrap()
}
