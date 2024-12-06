use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("example.txt")?;
    let reader = BufReader::new(file);

    let char_to_num = HashMap::from([
        ('^', 0),
        ('>', 1),
        ('v', 2),
        ('<', 3),
        ('#', 4),
        ('.', 5),
        ('X', 6),
        ('@', 7),
    ]);

    let next:Vec<(i64,i64)> = vec![(-1,0),(0,1),(1,0),(0,-1)];

    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<u8> = line.chars().map(|c| *char_to_num.get(&c).unwrap()).collect();
        grid.push(row);
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut bordered = vec![vec![7; width+2]; height+2];
    let mut y: usize = 1;
    let mut x: usize = 1;
    let mut direction:usize = 0;

    for i in 0..height {
        for j in 0..width {
            let item = grid[i][j] as usize;
            bordered[i+1][j+1] = item;
            if item < 4 {
                direction = item;
                x = j + 1;
                y = i + 1;
            }
        }
    }

    while bordered[y][x] != 7 {

        bordered[y][x] = 6;
        for _ in 0..4 {
            let test_y: usize = (y as i64 + next[direction].0) as usize;
            let test_x: usize = (x as i64 + next[direction].1) as usize;
            if bordered[test_y][test_x] == 4 {
                direction = (direction+1)%4;
            } else {
                x = test_x;
                y = test_y;
                break;
            }
        }
    }
    let mut total = 0;
    for i in 1..=height {
        for j in 1..=width {
            if bordered[i][j] == 6 {
                total += 1;
            }
        }
    }
    println!("{}", total);

    Ok(())
}