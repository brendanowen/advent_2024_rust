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
    let mut start:Vec<usize> = vec![0;indexes];
    for i in 0..indexes {
        for j in 0..digits[2*i] {
            data[next+j] = Some(i);
        }
        start[i] = next;
        next += digits[2*i] + digits[2*i+1];

    }

    for backwards in (0..indexes).rev() {

        let position:usize = start[backwards];
        let width:usize = digits[backwards*2];
        let mut free = 0;
        while free < position {
            if data[free].is_none() {
                let mut filled = free + 1;
                while data[filled].is_none() {
                    filled += 1;
                }
                let test_width = filled - free;
                if test_width >= width {
                    for i in 0..width {
                        data[free+i] = data[position+i];
                        data[position+i] = None;
                    }
                    break;
                }
                free = filled;
            } else {
                free += 1;
            }
        }
        
    }

    let mut total = 0;
    for i in 0..length {
        if let Some(value) = data[i] {
            total += i * value;
        }
    }



    println!("{}", total);

    Ok(())
}