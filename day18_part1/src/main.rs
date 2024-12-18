use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "data.txt"; // Replace with your file name
    let bytes = 1024;

    let data: Vec<(usize, usize)> = load_data_from_file(filename)?;
    let max_x: usize = data.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let max_y: usize = data.iter().map(|(_, y)| *y).max().unwrap() + 1;

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; max_x + 2]; max_y + 2];
    for x in 0..(max_x + 2) {
        grid[0][x] = '#';
        grid[max_y + 1][x] = '#';
    }
    for y in 0..(max_y + 2) {
        grid[y][0] = '#';
        grid[y][max_x + 1] = '#';
    }
    for i in 0..bytes {
        let (x, y) = data[i];
        grid[y + 1][x + 1] = '#';
    }

    let mut distances: Vec<Vec<Option<usize>>> = vec![vec![None; max_x + 2]; max_y + 2];

    distances[1][1] = Some(0);

    let mut changed = true;
    while changed {
        changed = false;
        for x in 1..(max_x + 1) {
            for y in 1..(max_y + 1) {
                if grid[y][x] == '#' {
                    continue;
                }
                if let Some(distance) = distances[y - 1][x] {
                    let compare = distance + 1;
                    if let Some(current) = distances[y][x] {
                        if compare < current {
                            distances[y][x] = Some(compare);
                            changed = true;
                        }
                    } else {
                        distances[y][x] = Some(compare);
                        changed = true;
                    }
                }
                if let Some(distance) = distances[y + 1][x] {
                    let compare = distance + 1;
                    if let Some(current) = distances[y][x] {
                        if compare < current {
                            distances[y][x] = Some(compare);
                            changed = true;
                        }
                    } else {
                        distances[y][x] = Some(compare);
                        changed = true;
                    }
                }
                if let Some(distance) = distances[y][x - 1] {
                    let compare = distance + 1;
                    if let Some(current) = distances[y][x] {
                        if compare < current {
                            distances[y][x] = Some(compare);
                            changed = true;
                        }
                    } else {
                        distances[y][x] = Some(compare);
                        changed = true;
                    }
                }
                if let Some(distance) = distances[y][x + 1] {
                    let compare = distance + 1;
                    if let Some(current) = distances[y][x] {
                        if compare < current {
                            distances[y][x] = Some(compare);
                            changed = true;
                        }
                    } else {
                        distances[y][x] = Some(compare);
                        changed = true;
                    }
                }
            }
        }
    }

    let distance = distances[max_y][max_x].unwrap();
    println!("{}", distance);
    Ok(())
}

fn load_data_from_file<P: AsRef<Path>>(
    filename: P,
) -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut data: Vec<(usize, usize)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() == 2 {
            let x: usize = parts[0].trim().parse::<usize>()?;
            let y: usize = parts[1].trim().parse::<usize>()?;
            data.push((x, y));
        } else {
            eprintln!("Warning: Skipping invalid line: {}", line);
        }
    }

    Ok(data)
}
