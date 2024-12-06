use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    let mut pairs: Vec<(u32, u32)> = Vec::new();
    let mut variable_length_vecs: Vec<Vec<u32>> = Vec::new();

    let mut is_first_part = true;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            is_first_part = false;
            continue;
        }

        if is_first_part {
            let mut iter = line.split('|');
            let first = iter.next().unwrap().parse::<u32>().unwrap();
            let second = iter.next().unwrap().parse::<u32>().unwrap();
            pairs.push((first, second));
        } else {
            let numbers: Vec<u32> = line.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
            variable_length_vecs.push(numbers);
        }
    }

    let mut total = 0;
    variable_length_vecs.iter().for_each(|list| {
        let length = list.len();
        if length%2 == 1 {
            let mut correct = true;
            'outer: for first in 0..length {

                for check_pair in &pairs {
                    if list[first] == check_pair.1 {
                        for second in (first+1)..length {
                            if check_pair.0 == list[second] {
                                correct = false;
                                break 'outer;
                            }
                        }        
                    }
                }
            }
            if correct {
                total += list[(length-1)/2];
            }
        }
    });

    println!("{total}");

    Ok(())
}