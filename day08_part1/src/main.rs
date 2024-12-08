use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("grid.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut char_map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for (x, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let row: Vec<char> = line.chars().collect();
        grid.push(row.clone());

        for (y, c) in row.iter().enumerate() {
            if *c != '.' && *c != '#' {
                char_map.entry(*c).or_insert(vec![]).push((x as i64, y as i64));
            }
        }
    }


    let width:i64 = grid.len() as i64;
    let height:i64 = grid[0].len() as i64;
    let mut total = 0;

    for (_char, positions) in char_map.iter() {
        
        for (x1, y1) in positions {
            for (x2, y2) in positions {
                if x1 != x2 || y1 != y2 {
                    let x3 = 2*x2 - x1;
                    let y3 = 2*y2 - y1;
                    if x3 < 0 || y3 < 0 || x3 >= width || y3 >= height {
                        continue;
                    }
                    if grid[x3 as usize][y3 as usize] != '@' {
                        grid[x3 as usize][y3 as usize] = '@';
                        total += 1;
                    }
                    
                } 
            }
        }
    }
    
    println!("{total}");
}