use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("numbers.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut total:i64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let str_numbers: Vec<&str> = line.split_whitespace().collect();
        let numbers:Vec<i64> = str_numbers.iter().map(|str_number| { let value:i64 = str_number.parse().unwrap(); value}).collect();
        let mut increasing = true;
        let mut decreasing = true;
        let mut too_much = false;

        let length = numbers.len();
        for i in 1..length {
            if numbers[i-1] <= numbers[i] {
                decreasing = false;
                let test_diff = numbers[i] - numbers[i-1];
                if test_diff > 3 {
                    too_much = true;
                }
            }
            if numbers[i-1] >= numbers[i] {
                increasing = false;
                let test_diff = numbers[i-1] - numbers[i];
                if test_diff > 3 {
                    too_much = true;
                }
            }
        }

        if !too_much && (increasing || decreasing) {
            total +=1;
        }

    }


    println!("Answer = {total}");
}