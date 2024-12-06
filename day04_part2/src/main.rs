use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("grid.txt")?;
    let reader = BufReader::new(file);

    let char_to_num = HashMap::from([
        ('X', 0),
        ('M', 1),
        ('A', 2),
        ('S', 3),
    ]);

    let mut lines: Vec<Vec<u64>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<u64> = line.chars().map(|c| *char_to_num.get(&c).unwrap_or(&4)).collect();
        lines.push(numbers);
    }

    let height: usize = lines.len()-2;
    let width: usize = lines[0].len()-2;

    let base:Vec<Vec<(usize,usize,usize)>> = vec![
        vec![
            (0,0,1),
            (0,2,3),
            (1,1,2),
            (2,0,1),
            (2,2,3)
        ],
        vec![
            (0,0,1),
            (0,2,1),
            (1,1,2),
            (2,0,3),
            (2,2,3)
        ],
        vec![
            (0,0,3),
            (0,2,1),
            (1,1,2),
            (2,0,3),
            (2,2,1)
        ],
        vec![
            (0,0,3),
            (0,2,3),
            (1,1,2),
            (2,0,1),
            (2,2,1)
        ],
    ];

    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            for group in &base {
                let mut correct = true;
                for &(x_extra,y_extra,value) in group {
                    if lines[y+y_extra][x+x_extra] != value as u64 {
                        correct = false;
                        break;
                    }
                }
                if correct {
                    total += 1;
                }    
            }
        }
    }

    println!("{total}");

    Ok(())
}