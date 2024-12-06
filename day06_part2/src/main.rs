use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("grid.txt")?;
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
        ('O', 8),
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

    let mut start_y: usize = 0;
    let mut start_x: usize = 0;
    let mut start_direction:usize = 0;

    'outer: for i in 0..height {
        for j in 0..width {
            let item = grid[i][j] as usize;
            if item < 4 {
                start_direction = item;
                start_x = j+1;
                start_y = i+1;
                break 'outer;
            }
        }
    }


    let mut total = 0;

    for obs_x in 1..=width {
        for obs_y in 1..=height {
            if obs_x == start_x && obs_y == start_y {
                continue;
            }

            let mut bordered = vec![vec![7; width+2]; height+2];
            let mut y: usize = start_y;
            let mut x: usize = start_x;
            let mut direction:usize = start_direction;
        
            for i in 0..height {
                for j in 0..width {
                    let item = grid[i][j] as usize;
                    bordered[i+1][j+1] = item;
                }
            }
            if bordered[obs_y][obs_x] != 5 {
                continue;
            }
            bordered[obs_y][obs_x] = 8;
            while bordered[y][x] != 7 {

                bordered[y][x] = direction;
                for _ in 0..4 {
                    let test_y: usize = (y as i64 + next[direction].0) as usize;
                    let test_x: usize = (x as i64 + next[direction].1) as usize;
                    if bordered[test_y][test_x] == 4 || bordered[test_y][test_x] == 8 {
                        direction = (direction+1)%4;
                    } else {
                        x = test_x;
                        y = test_y;
                        break;
                    }
                }
                if bordered[y][x] == direction {
                    total += 1;
                    break;
                }
                if bordered[y][x] == 7 {
                    break;
                }
            }

        }
    }
    


    println!("{}", total);

    Ok(())
}