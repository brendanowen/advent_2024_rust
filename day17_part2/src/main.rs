use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("problem.txt")?;
    let reader = BufReader::new(file);

    let mut program: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        match parts[0] {
            "Program" => {
                for value in parts[1].split(",") {
                    program.push(value.trim().parse().unwrap());
                }
            }
            _ => {}
        }
    }

    let length = program.len();
    let mut new_a = 0;
    for start_index in (0..length).rev() {
        let mut check_a = new_a * 8;
        loop {
            let mut a = check_a;
            let mut i = start_index;
            loop {
                if a == 0 {
                    break;
                }
                let b = a % 8;
                let b = b ^ 3;
                let c = a / (1 << b);
                let b = b ^ c;
                let b = b ^ 3;
                a = a / 8;
                if program[i] != b % 8 {
                    break;
                }
                i += 1;
            }
            if i >= length && a == 0 {
                break;
            }
            check_a += 1;
        }
        new_a = check_a;
    }

    println!("{new_a}");

    Ok(())
}
