// https://www.codewars.com/kata/58e24788e24ddee28e000053

use std::collections::HashMap;

fn main() {
    let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"]; // { "a" => 1 }
    let program2 = vec![
        "mov c 12",
        "mov b 0",
        "mov a 200",
        "dec a",
        "inc b",
        "jnz a -2",
        "dec c",
        "mov a b",
        "jnz c -5",
        "jnz 0 1",
        "mov c a",
    ]; // { "a" => 409600, "c" => 409600, "b" => 409600};

    println!("{:?}", simple_assembler(program));
    println!("{:?}", simple_assembler(program2));
}

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    // Run program
    let mut pc = 0usize;
    let mut registers: HashMap<String, i64> = HashMap::new();
    while pc < program.len() {
        let statement = program[pc].split_whitespace().collect::<Vec<_>>();
        match statement[..] {
            ["mov", x, y] => {
                let value = get_value(&registers, y);
                registers.insert(String::from(x), value);
            }
            ["inc", x] => *registers.get_mut(x).unwrap() += 1,
            ["dec", x] => *registers.get_mut(x).unwrap() -= 1,
            ["jnz", x, y] => {
                if get_value(&registers, x) != 0 {
                    pc = (pc as i64 + get_value(&registers, y)) as usize;
                    continue;
                }
            }
            _ => panic!("Unknown command / arg count"),
        }

        pc += 1;
    }

    registers
}

fn get_value(registers: &HashMap<String, i64>, arg: &str) -> i64 {
    match arg.parse::<i64>() {
        Ok(num) => num,
        Err(_) => registers[arg],
    }
}
