// https://www.codewars.com/kata/52a78825cdfc2cfc87000005

use regex::Regex;
use std::{collections::HashMap, hash::Hash};

fn main() {
    assert_eq!(calc("0"), 0.0);
    assert_eq!(calc("1"), 1.0);
    assert_eq!(calc("42"), 42.0);
    assert_eq!(calc("350"), 350.0);

    assert_eq!(calc("1 + 1"), 2.0);
    assert_eq!(calc("1 - 1"), 0.0);
    assert_eq!(calc("1 * 1"), 1.0);
    assert_eq!(calc("1 / 1"), 1.0);
    assert_eq!(calc("12 * 123"), 1476.0);

    assert_eq!(calc("1-1"), 0.0);
    assert_eq!(calc("1 -1"), 0.0);
    assert_eq!(calc("1- 1"), 0.0);
    assert_eq!(calc("1* 1"), 1.0);

    assert_eq!(calc("1- -1"), 2.0);
    assert_eq!(calc("1--1"), 2.0);
    assert_eq!(calc("1 - -1"), 2.0);
    assert_eq!(calc("-42"), -42.0);

    assert_eq!(calc("(1)"), 1.0);
    assert_eq!(calc("((1))"), 1.0);
    assert_eq!(calc("((80 - (19)))"), 61.0);

    assert_eq!(calc("12* 123/(-5 + 2)"), -492.0);
    assert_eq!(calc("1 - -(-(-(-4)))"), -3.0);
    assert_eq!(calc("2 /2+3 * 4.75- -6"), 21.25);
    assert_eq!(calc("2 / (2 + 3) * 4.33 - -6"), 7.732);
    assert_eq!(calc("(1 - 2) + -(-(-(-4)))"), 3.0);
    assert_eq!(calc("((2.33 / (2.9+3.5)*4) - -6)"), 7.45625);
}

fn calc(expr: &str) -> f64 {
    let tokens = tokenize(expr);
    let postfix = shunting_yard(tokens);
    eval(postfix)
}

#[derive(Debug)]
enum Token {
    Number(f64),
    Op(Operator),
    Negate,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

fn eval(postfix: Vec<Token>) -> f64 {
    let mut stack = Vec::new();

    for token in postfix {
        match &token {
            Token::Number(x) => stack.push(*x),
            Token::Op(op) => {
                let rhs = stack.pop();
                let lhs = stack.pop();
                match (lhs, rhs) {
                    (Some(x1), Some(x2)) => match op {
                        Operator::Add => stack.push(x1 + x2),
                        Operator::Sub => stack.push(x1 - x2),
                        Operator::Mul => stack.push(x1 * x2),
                        Operator::Div => stack.push(x1 / x2),
                    },
                    _ => panic!("Invalid operands for binary op"),
                }
            }
            Token::Negate => {
                let lhs = stack.pop();
                match lhs {
                    Some(x1) => stack.push(-x1),
                    _ => panic!("Invalid operand for negate"),
                }
            }
            _ => panic!("Unexpected token in postfix"),
        }
    }

    if stack.len() != 1 {
        panic!("Can't reduce expression");
    }

    *stack.first().unwrap()
}

fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let precedences = HashMap::from([
        (Operator::Add, 1),
        (Operator::Sub, 1),
        (Operator::Mul, 2),
        (Operator::Div, 2),
    ]);
    let mut stack = Vec::new();
    let mut output = Vec::new();

    for token in tokens {
        match &token {
            Token::Number(_) => output.push(token),
            Token::Op(op) => {
                let prec = precedences.get(&op).unwrap();
                while !stack.is_empty() {
                    let stack_token = stack.pop().unwrap();
                    match &stack_token {
                        Token::Op(stack_op) => {
                            let stack_token_prec = precedences.get(stack_op).unwrap();
                            if prec <= stack_token_prec {
                                output.push(stack_token);
                                continue;
                            }
                        }
                        Token::Negate => {
                            output.push(stack_token);
                            continue;
                        }
                        _ => {}
                    }

                    stack.push(stack_token);
                    break;
                }

                stack.push(token);
            }
            Token::Negate => stack.push(token),
            Token::LParen => stack.push(token),
            Token::RParen => loop {
                if stack.is_empty() {
                    panic!("Missing LParen");
                }

                match stack.pop().unwrap() {
                    Token::LParen => break,
                    stack_token => output.push(stack_token),
                }
            },
        }
    }

    while let Some(t) = stack.pop() {
        match t {
            Token::LParen => panic!("Missing RParen"),
            stack_token => output.push(stack_token),
        }
    }

    output
}

fn tokenize(input: &str) -> Vec<Token> {
    let rgx = Regex::new(r"([0-9]*\.?[0-9]+|[-+*/\(\)])").unwrap();
    let mut tokens = Vec::new();

    for m in rgx.find_iter(input) {
        tokens.push(match m.as_str().parse::<f64>() {
            Ok(num) => Token::Number(num),
            Err(_) => match m.as_str() {
                "+" => Token::Op(Operator::Add),
                "-" => match tokens.last() {
                    Some(Token::Op(_)) => Token::Negate,
                    Some(Token::LParen) => Token::Negate,
                    Some(_) => Token::Op(Operator::Sub),
                    None => Token::Negate,
                },
                "*" => Token::Op(Operator::Mul),
                "/" => Token::Op(Operator::Div),
                "(" => Token::LParen,
                ")" => Token::RParen,
                _ => panic!("Unknown token"),
            },
        });
    }

    tokens
}
