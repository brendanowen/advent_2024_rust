use std::collections::HashMap;
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

    let mut col2_counts: HashMap<i64, usize> = HashMap::new();

    // Count occurrences in col2
    for num in col2 {
        *col2_counts.entry(num).or_insert(0) += 1;
    }

    let total:i64 = col1.iter().map(|number| (col2_counts.get(number).cloned().unwrap_or(0) as i64)*(*number)).sum();

    println!("Answer = {total}");
}