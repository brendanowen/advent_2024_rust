use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("grid.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    let max_x = grid.len();
    let max_y = grid[0].len();

    let mut padded_grid: Vec<Vec<char>> = vec![vec!['.'; max_y + 2]; max_x + 2];
    for x in 0..max_x {
        for y in 0..max_y {
            padded_grid[x + 1][y + 1] = grid[x][y];
        }
    }

    let mut perimeter: Vec<Vec<usize>> = vec![vec![0; max_y + 2]; max_x + 2];
    for x in 1..=max_x {
        for y in 1..=max_y {
            if padded_grid[x][y] != padded_grid[x - 1][y] {
                perimeter[x][y] += 1;
            }
            if padded_grid[x][y] != padded_grid[x + 1][y] {
                perimeter[x][y] += 1;
            }
            if padded_grid[x][y] != padded_grid[x][y - 1] {
                perimeter[x][y] += 1;
            }
            if padded_grid[x][y] != padded_grid[x][y + 1] {
                perimeter[x][y] += 1;
            }
        }
    }

    let mut total = 0;

    for x in 1..=max_x {
        for y in 1..=max_y {
            if padded_grid[x][y] != '.' {
                let old = padded_grid[x][y];
                padded_grid[x][y] = '#';
                let mut change = true;
                while change {
                    change = false;
                    for x2 in 1..=max_x {
                        for y2 in 1..=max_y {
                            if padded_grid[x2][y2] == old
                                && (padded_grid[x2 - 1][y2] == '#'
                                    || padded_grid[x2 + 1][y2] == '#'
                                    || padded_grid[x2][y2 - 1] == '#'
                                    || padded_grid[x2][y2 + 1] == '#')
                            {
                                padded_grid[x2][y2] = '#';
                                change = true;
                            }
                        }
                    }
                }
                let mut area = 0;
                let mut total_perimeter = 0;
                for x2 in 1..=max_x {
                    for y2 in 1..=max_y {
                        if padded_grid[x2][y2] == '#' {
                            padded_grid[x2][y2] = '.';
                            area += 1;
                            total_perimeter += perimeter[x2][y2];
                        }
                    }
                }
                total += area * total_perimeter;
            }
        }
    }

    println!("{total}");

    Ok(())
}
