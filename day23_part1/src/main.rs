use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("data.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut pairs: Vec<([char; 2], [char; 2], bool)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.len() == 5 && line.chars().nth(2) == Some('-') {
            let left: [char; 2] = line[0..2]
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            let right: [char; 2] = line[3..5]
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();

            let has_t = left[0] == 't' || right[0] == 't';

            if left.cmp(&right) == Ordering::Less {
                pairs.push((left, right, has_t));
            } else {
                pairs.push((right, left, has_t));
            }
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }

    let mut total = 0;
    let number = pairs.len();
    for p1 in 0..number {
        let pair1 = pairs[p1];
        for p2 in (p1 + 1)..number {
            let pair2 = pairs[p2];
            if !pair1.2 && !pair2.2 {
                continue;
            }
            let joins = if pair1.0 == pair2.0 {
                Some((pair1.0, pair1.1, pair2.1))
            } else if pair1.0 == pair2.1 {
                Some((pair1.0, pair1.1, pair2.0))
            } else if pair1.1 == pair2.0 {
                Some((pair1.1, pair1.0, pair2.1))
            } else if pair1.1 == pair2.1 {
                Some((pair1.1, pair1.0, pair2.0))
            } else {
                None
            };
            if let Some(join) = joins {
                for p3 in (p2 + 1)..number {
                    let pair3 = pairs[p3];
                    if (pair3.0 == join.1 && pair3.1 == join.2)
                        || (pair3.0 == join.2 && pair3.1 == join.1)
                    {
                        total += 1;
                    }
                }
            }
        }
    }
    println!("{total}");

    Ok(())
}
