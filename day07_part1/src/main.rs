use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("calculations.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(": ");

        let key_str = parts.next().unwrap();
        let values_str = parts.next().unwrap();

        let key: i64 = key_str.parse().unwrap();
        let values: Vec<i64> = values_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut unique_values = HashSet::new();
        unique_values.insert(values[0]);
        for next in values.iter().skip(1) {

            let mut next_values = HashSet::new();
            for previous in unique_values {
                next_values.insert(previous+next);
                next_values.insert(previous*next);  
            }
            unique_values = next_values;
        }
        for final_value in unique_values {
            if final_value == key {
                total += key;
                break;
            }
        }
    }

    println!("{}", total); // 6230999301795
}