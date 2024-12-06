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

    let height: usize = lines.len();
    let width: usize = lines[0].len();

    let step:Vec<(i64,i64,usize,usize,usize,usize)> = vec![
        (0,1,0,height,0,width-3),
        (0,-1,0,height,3,width),
        (1,0,0,height-3,0,width),
        (-1,0,3,height,0,width),
        (1,1,0,height-3,0,width-3),
        (1,-1,0,height-3,3,width),
        (-1,1,3,height,0,width-3),
        (-1,-1,3,height,3,width)];

    let mut total = 0;
    for &(step_y,step_x,start_y,end_y,start_x,end_x) in step.iter() {
        for y in start_y..end_y {
            for x in start_x..end_x {
                let mut correct = true;
                for i in 0..4 {
                    if lines[((y as i64)+step_y*i) as usize][((x as i64)+step_x*i) as usize] != i as u64 {
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