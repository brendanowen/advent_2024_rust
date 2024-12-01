use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("numbers.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut col1: Vec<i64> = Vec::new();
    let mut col2: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() == 2 {
            let num1: i64 = numbers[0].parse().unwrap();
            let num2: i64 = numbers[1].parse().unwrap();
            col1.push(num1);
            col2.push(num2);
        }
    }

    col1.sort();
    col2.sort();

    let total = col1.iter().zip(col2).map(|(value1, value2)| (value1-value2).abs()).sum::<i64>();

    println!("Answer = {total}");
}