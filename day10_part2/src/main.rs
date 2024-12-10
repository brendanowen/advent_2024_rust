use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("grid.txt").expect("File not found");
    let reader = BufReader::new(file);

    let grid: Vec<Vec<usize>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let width = grid.len();
    let height = grid[0].len();

    let mut larger: Vec<Vec<usize>> = vec![vec![10; height + 2]; width + 2];
    for x in 0..width {
        for y in 0..height {
            larger[x + 1][y + 1] = grid[x][y];
        }
    }

    let mut total = 0;

    for x in 1..=width {
        for y in 1..=height {
            if larger[x][y] == 0 {
                let mut grow = vec![vec![10; height + 2]; width + 2];
                let mut counts = vec![vec![0; height + 2]; width + 2];
                grow[x][y] = 0;
                counts[x][y] = 1;
                for digit in 1..=9 {
                    let previous = digit - 1;
                    let mut count = 0;
                    for check_x in 1..=width {
                        for check_y in 1..=height {
                            if larger[check_x][check_y] == digit {
                                if grow[check_x - 1][check_y] == previous {
                                    counts[check_x][check_y] += counts[check_x - 1][check_y];
                                }
                                if grow[check_x + 1][check_y] == previous {
                                    counts[check_x][check_y] += counts[check_x + 1][check_y];
                                }
                                if grow[check_x][check_y - 1] == previous {
                                    counts[check_x][check_y] += counts[check_x][check_y - 1];
                                }
                                if grow[check_x][check_y + 1] == previous {
                                    counts[check_x][check_y] += counts[check_x][check_y + 1];
                                }
                                if counts[check_x][check_y] > 0 {
                                    grow[check_x][check_y] = digit;
                                    count += counts[check_x][check_y];
                                }
                            }
                        }
                    }
                    if digit == 9 {
                        total += count;
                    }
                }
            }
        }
    }

    println!("{}", total);
}
