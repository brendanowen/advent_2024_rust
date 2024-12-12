use std::fs::File;
use std::io::{BufRead, BufReader};

fn split_or_multiply(num: usize) -> Vec<usize> {
    let num_str = num.to_string();
    let len = num_str.len();

    if num == 0 {
        return vec![1];
    } else if len % 2 == 0 {
        let mid = len / 2;
        let first_half = num_str[..mid].parse::<usize>().unwrap();
        let second_half = num_str[mid..].parse::<usize>().unwrap();
        return vec![first_half, second_half];
    } else {
        return vec![num * 2024];
    }
}

fn main() {
    let file = File::open("values.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut values: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        for num_str in line.split_whitespace() {
            let num = num_str.parse::<usize>().expect("Could not parse number");
            values.push(num);
        }
    }

    for _ in 0..25 {
        let mut next_values: Vec<usize> = Vec::new();
        values.iter().for_each(|value| {
            let new_items = split_or_multiply(*value);
            new_items.iter().for_each(|new_value| {
                next_values.push(*new_value);
            });
        });
        values = next_values;
    }

    println!("{}", values.len());
}
