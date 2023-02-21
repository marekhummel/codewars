// https://www.codewars.com/kata/58e61f3d8ff24f774400002c

use regex::Regex;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let simple_programs = &[
            "\n; My first program\nmov  a, 5\ninc  a\ncall function\nmsg  '(5+1)/2 = ', a    ; output message\nend\n\nfunction:\n    div  a, 2\n    ret\n",
            "\nmov   a, 5\nmov   b, a\nmov   c, a\ncall  proc_fact\ncall  print\nend\n\nproc_fact:\n    dec   b\n    mul   c, b\n    cmp   b, 1\n    jne   proc_fact\n    ret\n\nprint:\n    msg   a, '! = ', c ; output text\n    ret\n",
            "\nmov   a, 8            ; value\nmov   b, 0            ; next\nmov   c, 0            ; counter\nmov   d, 0            ; first\nmov   e, 1            ; second\ncall  proc_fib\ncall  print\nend\n\nproc_fib:\n    cmp   c, 2\n    jl    func_0\n    mov   b, d\n    add   b, e\n    mov   d, e\n    mov   e, b\n    inc   c\n    cmp   c, a\n    jle   proc_fib\n    ret\n\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n\nprint:\n    msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text\n    ret\n",
            "\nmov   a, 11           ; value1\nmov   b, 3            ; value2\ncall  mod_func\nmsg   'mod(', a, ', ', b, ') = ', d        ; output\nend\n\n; Mod function\nmod_func:\n    mov   c, a        ; temp1\n    div   c, b\n    mul   c, b\n    mov   d, a        ; temp2\n    sub   d, c\n    ret\n",
            "\nmov   a, 81         ; value1\nmov   b, 153        ; value2\ncall  init\ncall  proc_gcd\ncall  print\nend\n\nproc_gcd:\n    cmp   c, d\n    jne   loop\n    ret\n\nloop:\n    cmp   c, d\n    jg    a_bigger\n    jmp   b_bigger\n\na_bigger:\n    sub   c, d\n    jmp   proc_gcd\n\nb_bigger:\n    sub   d, c\n    jmp   proc_gcd\n\ninit:\n    cmp   a, 0\n    jl    a_abs\n    cmp   b, 0\n    jl    b_abs\n    mov   c, a            ; temp1\n    mov   d, b            ; temp2\n    ret\n\na_abs:\n    mul   a, -1\n    jmp   init\n\nb_abs:\n    mul   b, -1\n    jmp   init\n\nprint:\n    msg   'gcd(', a, ', ', b, ') = ', c\n    ret\n",
            "\ncall  func1\ncall  print\nend\n\nfunc1:\n    call  func2\n    ret\n\nfunc2:\n    ret\n\nprint:\n    msg 'This program should return null'\n",
            "\nmov   a, 2            ; value1\nmov   b, 10           ; value2\nmov   c, a            ; temp1\nmov   d, b            ; temp2\ncall  proc_func\ncall  print\nend\n\nproc_func:\n    cmp   d, 1\n    je    continue\n    mul   c, a\n    dec   d\n    call  proc_func\n\ncontinue:\n    ret\n\nprint:\n    msg a, '^', b, ' = ', c\n    ret\n"];

    let expected = &[
        Some(String::from("(5+1)/2 = 3")),
        Some(String::from("5! = 120")),
        Some(String::from("Term 8 of Fibonacci series is: 21")),
        Some(String::from("mod(11, 3) = 2")),
        Some(String::from("gcd(81, 153) = 9")),
        None,
        Some(String::from("2^10 = 1024")),
    ];

    for (prg, exp) in simple_programs.iter().zip(expected) {
        let actual = AssemblerInterpreter::interpret(*prg);
        assert_eq!(actual, *exp);
        println!("{actual:?}");
    }
}

pub struct AssemblerInterpreter {}

impl AssemblerInterpreter {
    pub fn interpret(input: &str) -> Option<String> {
        let rgx_label = Regex::new(r"^([a-z0-9_]+):(?: *;.*)?$").unwrap();
        let rgx_statement = Regex::new(r"^ *([a-z]+) *((?:\w+|'.*?')(?:, *(?:\w+|'.*?'))*)?(?: *;.*)?$").unwrap();
        let rgx_arg = Regex::new(r"\w+|'.*?'").unwrap();
        let rgx_string_literal = Regex::new(r"^'(.*)'$").unwrap();

        let program = input.lines().collect::<Vec<_>>();

        // Find labels / functions
        let labels = program
            .iter()
            .enumerate()
            .map(|(i, s)| (i, rgx_label.captures(s)))
            .filter(|(_, m)| m.is_some())
            .map(|(i, m)| (m.unwrap().get(1).unwrap().as_str(), i))
            .collect::<HashMap<_, _>>();

        // Run program
        let mut pc = 0usize;
        let mut registers: HashMap<&str, i32> = HashMap::new();
        let mut compare_flag: Option<Ordering> = None;
        let mut return_stack = Vec::new();
        let mut output = String::new();
        while pc < program.len() {
            let possible_statement = rgx_statement.captures(program[pc]);
            if possible_statement.is_none() {
                pc += 1;
                continue;
            }

            let stmt_captures = possible_statement.unwrap();
            let command = stmt_captures.get(1).unwrap().as_str();
            let args = stmt_captures.get(2).map_or(Vec::new(), |m| {
                rgx_arg.find_iter(m.as_str()).map(|m2| m2.as_str()).collect()
            });

            match (command, &args[..]) {
                ("mov", [x, y]) => {
                    let value = AssemblerInterpreter::get_value(&registers, y);
                    registers.insert(x, value);
                }
                ("inc", [x]) => {
                    registers.insert(x, registers[x] + 1);
                }
                ("dec", [x]) => {
                    registers.insert(x, registers[x] - 1);
                }
                ("add", [x, y]) => {
                    let value = AssemblerInterpreter::get_value(&registers, y);
                    registers.insert(x, registers[x] + value);
                }
                ("sub", [x, y]) => {
                    let value = AssemblerInterpreter::get_value(&registers, y);
                    registers.insert(x, registers[x] - value);
                }
                ("mul", [x, y]) => {
                    let value = AssemblerInterpreter::get_value(&registers, y);
                    registers.insert(x, registers[x] * value);
                }
                ("div", [x, y]) => {
                    let value = AssemblerInterpreter::get_value(&registers, y);
                    registers.insert(x, registers[x] / value);
                }
                ("jmp", [lbl]) => {
                    pc = *labels.get(lbl).unwrap();
                }
                ("cmp", [x, y]) => {
                    let value_x = AssemblerInterpreter::get_value(&registers, x);
                    let value_y = AssemblerInterpreter::get_value(&registers, y);
                    compare_flag = Some(value_x.cmp(&value_y));
                }
                ("jne", [lbl]) => {
                    if compare_flag.is_some() && compare_flag.unwrap().is_ne() {
                        pc = *labels.get(lbl).unwrap();
                    }
                }
                ("je", [lbl]) => {
                    if compare_flag.is_some() && compare_flag.unwrap().is_eq() {
                        pc = *labels.get(lbl).unwrap();
                    }
                }
                ("jge", [lbl]) => {
                    if compare_flag.is_some() && compare_flag.unwrap().is_ge() {
                        pc = *labels.get(lbl).unwrap();
                    }
                }
                ("jg", [lbl]) => {
                    if compare_flag.is_some() && compare_flag.unwrap().is_gt() {
                        pc = *labels.get(lbl).unwrap();
                    }
                }
                ("jle", [lbl]) => {
                    if compare_flag.is_some() && compare_flag.unwrap().is_le() {
                        pc = *labels.get(lbl).unwrap();
                    }
                }
                ("jl", [lbl]) => {
                    if compare_flag.is_some() && compare_flag.unwrap().is_lt() {
                        pc = *labels.get(lbl).unwrap();
                    }
                }
                ("call", [lbl]) => {
                    return_stack.push(pc);
                    pc = *labels.get(lbl).unwrap();
                }
                ("ret", []) => {
                    pc = return_stack.pop().unwrap();
                }
                ("msg", args) => {
                    for arg in args {
                        let msg = match rgx_string_literal.captures(arg) {
                            Some(captures) => captures.get(1).unwrap().as_str().to_owned(),
                            None => AssemblerInterpreter::get_value(&registers, arg).to_string(),
                        };
                        output.push_str(&msg);
                    }
                }
                ("end", []) => return Some(output),
                _ => panic!("Unknown command / arg count"),
            }

            pc += 1;
        }

        None
    }

    fn get_value(registers: &HashMap<&str, i32>, arg: &str) -> i32 {
        match arg.parse::<i32>() {
            Ok(num) => num,
            Err(_) => registers[arg],
        }
    }
}
