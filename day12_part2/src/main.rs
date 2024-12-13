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
                let mut segments: Vec<(usize, usize, usize, usize, usize)> = vec![];
                let mut area = 0;
                for x2 in 1..=max_x {
                    for y2 in 1..=max_y {
                        if padded_grid[x2][y2] == '#' {
                            area += 1;

                            if padded_grid[x2 - 1][y2] != '#' {
                                segments.push((0, x2 - 1, y2 - 1, x2 - 1, y2));
                            }
                            if padded_grid[x2 + 1][y2] != '#' {
                                segments.push((1, x2, y2 - 1, x2, y2));
                            }
                            if padded_grid[x2][y2 - 1] != '#' {
                                segments.push((2, x2 - 1, y2 - 1, x2, y2 - 1));
                            }
                            if padded_grid[x2][y2 + 1] != '#' {
                                segments.push((3, x2 - 1, y2, x2, y2));
                            }
                        }
                    }
                }
                for x2 in 1..=max_x {
                    for y2 in 1..=max_y {
                        if padded_grid[x2][y2] == '#' {
                            padded_grid[x2][y2] = '.';
                        }
                    }
                }
                let perimeter: usize = segments.len();
                let mut used: Vec<bool> = vec![false; perimeter];
                let mut fences: usize = 0;
                for i in 0..perimeter {
                    if !used[i] {
                        used[i] = true;
                        fences += 1;
                        let edge: &(usize, usize, usize, usize, usize) = &segments[i];
                        let mut start_x = edge.1;
                        let mut start_y = edge.2;
                        let mut end_x = edge.3;
                        let mut end_y = edge.4;
                        let mut joined = true;
                        while joined {
                            joined = false;
                            for j in 0..perimeter {
                                if !used[j] {
                                    let test_edge: &(usize, usize, usize, usize, usize) =
                                        &segments[j];
                                    if test_edge.0 == edge.0 {
                                        if end_x == test_edge.1 && end_y == test_edge.2 {
                                            used[j] = true;
                                            end_x = test_edge.3;
                                            end_y = test_edge.4;
                                            joined = true;
                                        } else if start_x == test_edge.3 && start_y == test_edge.4 {
                                            used[j] = true;
                                            start_x = test_edge.1;
                                            start_y = test_edge.2;
                                            joined = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                total += area * fences;
            }
        }
    }

    println!("{total}");

    Ok(())
}
