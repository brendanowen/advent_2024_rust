use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Read the file (or use a string literal for testing)
    let lines = read_lines("data.txt")?; // Replace "example.txt" with your file path

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start = None;
    let mut end = None;

    for (row_index, line) in lines.iter().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (col_index, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'S' {
                start = Some((row_index, col_index));
            } else if c == 'E' {
                end = Some((row_index, col_index));
            }
        }
        map.push(row);
    }

    let width: usize = map[0].len();
    let height: usize = map.len();
    let adjacents: Vec<(i64, i64)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

    if let Some((row_s, col_s)) = start {
        if let Some((row_e, col_e)) = end {
            let mut distances_forward: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
            distances_forward[row_s][col_s] = Some(0);

            let mut distances_backward: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
            distances_backward[row_e][col_e] = Some(0);

            let mut locations: Vec<(usize, usize)> = vec![];
            for row in 0..height {
                for col in 0..width {
                    if map[row][col] != '#' {
                        locations.push((row, col));
                    }
                }
            }

            let mut changed = true;
            while changed {
                changed = false;
                for location in &locations {
                    let mut min_distance_forward =
                        distances_forward[location.0][location.1].unwrap_or(width * height + 1);
                    let mut min_distance_backward =
                        distances_backward[location.0][location.1].unwrap_or(width * height + 1);
                    for adjacent in &adjacents {
                        let row = (location.0 as i64 + adjacent.0) as usize;
                        let col = (location.1 as i64 + adjacent.1) as usize;
                        if map[row][col] != '#' {
                            if let Some(test_distance) = distances_forward[row][col] {
                                if test_distance + 1 < min_distance_forward {
                                    min_distance_forward = test_distance + 1;
                                    distances_forward[location.0][location.1] =
                                        Some(min_distance_forward);
                                    changed = true;
                                }
                            }
                            if let Some(test_distance) = distances_backward[row][col] {
                                if test_distance + 1 < min_distance_backward {
                                    min_distance_backward = test_distance + 1;
                                    distances_backward[location.0][location.1] =
                                        Some(min_distance_backward);
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
            if let Some(final_distance) = distances_backward[row_s][col_s] {
                for row_w in 1..(height - 1) {
                    for col_w in 1..(width - 1) {
                        if map[row_w][col_w] == '#' {
                            let mut min_distance_forward = width * height + 1;
                            for adjacent in &adjacents {
                                let row = (row_w as i64 + adjacent.0) as usize;
                                let col = (col_w as i64 + adjacent.1) as usize;
                                if map[row][col] != '#' {
                                    if let Some(test_distance) = distances_forward[row][col] {
                                        if test_distance + 1 < min_distance_forward {
                                            min_distance_forward = test_distance + 1;
                                            distances_forward[row_w][col_w] =
                                                Some(min_distance_forward);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                let mut linked: Vec<(usize, usize, usize, usize)> = vec![];

                for row in 1..(height - 2) {
                    for col in 1..(width - 1) {
                        if map[row][col] == '#' && map[row + 1][col] != '#' {
                            linked.push((row, col, row + 1, col));
                        }
                        if map[row + 1][col] == '#' && map[row][col] != '#' {
                            linked.push((row + 1, col, row, col));
                        }
                    }
                }
                for row in 1..(height - 1) {
                    for col in 1..(width - 2) {
                        if map[row][col] == '#' && map[row][col + 1] != '#' {
                            linked.push((row, col, row, col + 1));
                        }
                        if map[row][col + 1] == '#' && map[row][col] != '#' {
                            linked.push((row, col + 1, row, col));
                        }
                    }
                }

                let mut total = 0;
                let mut counts: Vec<usize> = vec![0; final_distance + 1];
                for link in linked {
                    if let Some(previous) = distances_forward[link.0][link.1] {
                        if let Some(ahead) = distances_backward[link.2][link.3] {
                            let cost = previous + ahead + 1;
                            if cost < final_distance {
                                let difference = final_distance - cost;
                                if difference >= 100 {
                                    total += 1;
                                }
                                counts[difference] += 1;
                            }
                        }
                    }
                }

                println!("{total}");
            }
        }
    }

    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}
