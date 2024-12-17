use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("problem.txt")?;
    let reader = BufReader::new(file);

    let mut reg_a: i64 = 0;
    let mut reg_b: i64 = 0;
    let mut reg_c: i64 = 0;
    let mut program: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        match parts[0] {
            "Register A" => reg_a = parts[1].parse().unwrap(),
            "Register B" => reg_b = parts[1].parse().unwrap(),
            "Register C" => reg_c = parts[1].parse().unwrap(),
            "Program" => {
                for value in parts[1].split(",") {
                    program.push(value.trim().parse().unwrap());
                }
            }
            _ => {}
        }
    }

    let mut next = false;

    let mut instruction_pointer = 0;
    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let literal = program[instruction_pointer + 1];
        let combo = match literal {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            value => value,
        };
        instruction_pointer += 2;
        match opcode {
            0 => {
                let numerator = reg_a;
                let denominator = 1 << combo;
                reg_a = numerator / denominator;
            }
            1 => {
                reg_b = reg_b ^ literal;
            }
            2 => {
                reg_b = combo % 8;
            }
            3 => {
                if reg_a != 0 {
                    instruction_pointer = literal as usize;
                }
            }
            4 => {
                reg_b = reg_b ^ reg_c;
            }
            5 => {
                if next {
                    print!(",");
                } else {
                    next = true;
                }
                print!("{}", combo % 8);
            }
            6 => {
                let numerator = reg_a;
                let denominator = 1 << combo;
                reg_b = numerator / denominator;
            }
            7 => {
                let numerator = reg_a;
                let denominator = 1 << combo;
                reg_c = numerator / denominator;
            }
            _ => {}
        }
    }
    println!();

    Ok(())
}
