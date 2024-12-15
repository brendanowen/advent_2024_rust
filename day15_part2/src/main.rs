use std::fs;

fn main() {
    let contents =
        fs::read_to_string("problem.txt").expect("Something went wrong reading the file");

    let mut small_grid: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<char> = Vec::new();

    let mut in_grid = true;
    for line in contents.lines() {
        if line
            .chars()
            .all(|c| c == '#' || c == '.' || c == 'O' || c == '@')
        {
            if in_grid {
                let lines: Vec<char> = line.chars().collect();
                if !lines.is_empty() {
                    small_grid.push(line.chars().collect());
                }
            }
        } else {
            in_grid = false;
            instructions.extend(line.chars());
        }
    }

    let height = small_grid.len();
    let width = small_grid[0].len();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width * 2]; height];

    for y in 0..height {
        for x in 0..width {
            match small_grid[y][x] {
                '#' => {
                    grid[y][2 * x] = '#';
                    grid[y][2 * x + 1] = '#';
                }
                '.' => {
                    grid[y][2 * x] = '.';
                    grid[y][2 * x + 1] = '.';
                }
                'O' => {
                    grid[y][2 * x] = '[';
                    grid[y][2 * x + 1] = ']';
                }
                '@' => {
                    grid[y][2 * x] = '@';
                    grid[y][2 * x + 1] = '.';
                }
                _ => {}
            }
        }
    }

    let width = width * 2;
    let mut robot_y = 0;
    let mut robot_x = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '@' {
                robot_y = y;
                robot_x = x;
            }
        }
    }

    instructions
        .iter()
        .for_each(|instruction| match instruction {
            '<' => {
                let mut gap_x = robot_x - 1;
                while grid[robot_y][gap_x] != '.' && grid[robot_y][gap_x] != '#' {
                    gap_x -= 1;
                }
                if grid[robot_y][gap_x] != '#' {
                    for x in gap_x..robot_x {
                        grid[robot_y][x] = grid[robot_y][x + 1];
                    }
                    grid[robot_y][robot_x] = '.';
                    robot_x -= 1;
                }
            }
            'v' => {
                let mut y = robot_y;
                let mut to_move: Vec<Vec<bool>> = vec![vec![false; width]; height];
                to_move[robot_y][robot_x] = true;

                let mut stuck = false;
                while y < height {
                    let mut row_cleared = true;
                    for x in 0..width {
                        if to_move[y][x] && grid[y + 1][x] != '.' {
                            row_cleared = false;
                            break;
                        }
                    }
                    if row_cleared {
                        break;
                    }
                    for x in 1..(width - 1) {
                        if to_move[y][x] {
                            if grid[y + 1][x] == ']' {
                                to_move[y + 1][x] = true;
                                to_move[y + 1][x - 1] = true;
                            } else if grid[y + 1][x] == '[' {
                                to_move[y + 1][x] = true;
                                to_move[y + 1][x + 1] = true;
                            } else if grid[y + 1][x] == '#' {
                                stuck = true;
                            }
                        }
                    }
                    if stuck {
                        break;
                    }

                    y += 1;
                }

                if !stuck {
                    for y in (robot_y..(height - 1)).rev() {
                        for x in 0..width {
                            if to_move[y][x] {
                                grid[y + 1][x] = grid[y][x];
                                grid[y][x] = '.';
                            }
                        }
                    }
                    robot_y += 1;
                }
            }
            '>' => {
                let mut gap_x = robot_x + 1;
                while grid[robot_y][gap_x] != '.' && grid[robot_y][gap_x] != '#' {
                    gap_x += 1;
                }
                if grid[robot_y][gap_x] != '#' {
                    for x in (robot_x..=gap_x).rev() {
                        grid[robot_y][x] = grid[robot_y][x - 1];
                    }
                    grid[robot_y][robot_x] = '.';
                    robot_x += 1;
                }
            }
            '^' => {
                let mut y = robot_y;
                let mut to_move: Vec<Vec<bool>> = vec![vec![false; width]; height];
                to_move[robot_y][robot_x] = true;

                let mut stuck = false;
                loop {
                    let mut row_cleared = true;
                    for x in 0..width {
                        if to_move[y][x] && grid[y - 1][x] != '.' {
                            row_cleared = false;
                            break;
                        }
                    }
                    if row_cleared {
                        break;
                    }
                    for x in 1..(width - 1) {
                        if to_move[y][x] {
                            if grid[y - 1][x] == ']' {
                                to_move[y - 1][x] = true;
                                to_move[y - 1][x - 1] = true;
                            } else if grid[y - 1][x] == '[' {
                                to_move[y - 1][x] = true;
                                to_move[y - 1][x + 1] = true;
                            } else if grid[y - 1][x] == '#' {
                                stuck = true;
                            }
                        }
                    }
                    if stuck {
                        break;
                    }

                    if y == 0 {
                        break;
                    }
                    y -= 1;
                }

                if !stuck {
                    for y in 1..=robot_y {
                        for x in 0..width {
                            if to_move[y][x] {
                                grid[y - 1][x] = grid[y][x];
                                grid[y][x] = '.';
                            }
                        }
                    }
                    robot_y -= 1;
                }
            }
            _ => {}
        });

    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '[' {
                total += 100 * y + x;
            }
        }
    }

    println!("{total}")
}
