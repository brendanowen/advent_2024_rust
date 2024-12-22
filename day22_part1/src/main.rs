use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "data.txt";

    let numbers = read_numbers(file_path)?;

    let mut total = 0;
    for i in numbers {
        let mut current: u64 = i;
        for _ in 0..2000 {
            let x1: u64 = ((current * 64) ^ current) % 16777216;
            let x2: u64 = ((x1 / 32) ^ x1) % 16777216;
            let x3: u64 = ((x2 * 2048) ^ x2) % 16777216;

            current = x3;
        }
        total += current;
    }

    println!("{total}");
    Ok(())
}

fn read_numbers<P>(filename: P) -> io::Result<Vec<u64>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut numbers: Vec<u64> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        match line.parse::<u64>() {
            Ok(number) => numbers.push(number),
            Err(err) => {
                eprintln!("Error parsing line '{}': {}", line, err);
                // Consider returning an error here if you want to stop on parsing errors.
                // For example: return Err(io::Error::new(io::ErrorKind::InvalidData, err));
            }
        }
    }

    Ok(numbers)
}
