use std::{fs::File, io::Read};

use regex::Regex;

fn split_by_do_dont(input: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut i = 0;
    let mut in_do_dont = true;

    while i < input.len() {
        if  input[i..].starts_with("don't()") {
            // Handle the initial part before the first "don't()"
            if in_do_dont {
                result.push(&input[start..i]);
            }
            start = i + "don't()".len();
            i = start;
            in_do_dont = false;
        } else if input[i..].starts_with("do()") {            
            if in_do_dont {
                result.push(&input[start..i]);
            }
            start = i + "do()".len();
            i = start;
            in_do_dont = true;
        }
        i += 1;
    }

    // Handle the remaining part after the last "don't()" or "do()"
    if start < input.len() {

        if in_do_dont {
            result.push(&input[start..]);
        }
    }

    result
}

fn extract_total_mult(input: &str) ->i64 {
    let regex: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<i64>().unwrap();
            let b = cap[2].parse::<i64>().unwrap();
            a * b
        })
        .sum()
}


fn main() {    
    let mut file = File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let result = split_by_do_dont(&contents);
    let mut total = 0;
    result.iter().for_each(|item| total += extract_total_mult(item));

    println!("{:?}", total);
}