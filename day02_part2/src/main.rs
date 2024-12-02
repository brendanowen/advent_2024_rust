use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("numbers.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut total:i64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let str_numbers: Vec<&str> = line.split_whitespace().collect();
        let original:Vec<i64> = str_numbers.iter().map(|str_number| { let value:i64 = str_number.parse().unwrap(); value}).collect();
        let mut numbers = original.clone();

        let old_length = original.len();
        let length = original.len()-1;
        
        for remove in 0..old_length {
            let swap_new = old_length-remove-1;
            if remove != 0 {
                numbers[swap_new] = original[swap_new+1];
            }

            let mut increasing = true;
            let mut decreasing = true;
            let mut too_much = false;
    
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
                break;
            }
    

        }
        
        
    }


    println!("Answer = {total}");
}