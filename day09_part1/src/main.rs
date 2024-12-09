use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    let line = reader.lines().next().unwrap()?;
    let mut digits: Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    digits.push(0);
    let length:usize = digits.iter().sum();
    let mut data:Vec<Option<usize>> = vec![None; length];
    let indexes:usize = digits.len()/2;
    let mut next:usize = 0;
    for i in 0..indexes {
        for j in 0..digits[2*i] {
            data[next+j] = Some(i);
        }
        next += digits[2*i] + digits[2*i+1];
    }

    let mut start = 0;
    let mut end = length -1;
    while start < end {
        if data[start].is_some() {
            start += 1;
        } else if data[end].is_none() {
            end -=1;
        } else  {
            data[start] = data[end];
            data[end] = None;
            start += 1;
            end -= 1;
        } 
    }

    let mut total = 0;
    for i in 0..length {
        if let Some(value) = data[i] {
            total += i * value;
        } else {
            break;
        }
    }



    println!("{}", total);

    Ok(())
}