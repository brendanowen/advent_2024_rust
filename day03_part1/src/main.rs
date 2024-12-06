use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn extract_paris(input: &str) -> Vec<(i64, i64)> {
    let regex: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<i64>().unwrap();
            let b = cap[2].parse::<i64>().unwrap();
            (a, b)
        })
        .collect()
}


fn main() {
    let file = File::open("data.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut total:i64 = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        let pairs = extract_paris(&line);
        pairs.iter().for_each(|&(a,b)| total += a*b);
    }


    println!("Answer = {total}");
}