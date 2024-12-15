use std::fs;

fn main() {
    let contents =
        fs::read_to_string("problem.txt").expect("Something went wrong reading the file");

    let mut grid: Vec<Vec<char>> = Vec::new();
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
                    grid.push(line.chars().collect());
                }
            }
        } else {
            in_grid = false;
            instructions.extend(line.chars());
        }
    }

    let height = grid.len();
    let width = grid[0].len();

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
                let mut gap_x = robot_x;
                while grid[robot_y][gap_x] != '.' && grid[robot_y][gap_x] != '#' {
                    gap_x -= 1;
                }
                if grid[robot_y][gap_x] == '.' {
                    grid[robot_y][gap_x] = 'O';
                    grid[robot_y][robot_x] = '.';
                    grid[robot_y][robot_x - 1] = '@';
                    robot_x -= 1;
                }
            }
            'v' => {
                let mut gap_y = robot_y;
                while grid[gap_y][robot_x] != '.' && grid[gap_y][robot_x] != '#' {
                    gap_y += 1;
                }
                if grid[gap_y][robot_x] == '.' {
                    grid[gap_y][robot_x] = 'O';
                    grid[robot_y][robot_x] = '.';
                    grid[robot_y + 1][robot_x] = '@';
                    robot_y += 1;
                }
            }
            '>' => {
                let mut gap_x = robot_x;
                while grid[robot_y][gap_x] != '.' && grid[robot_y][gap_x] != '#' {
                    gap_x += 1;
                }
                if grid[robot_y][gap_x] == '.' {
                    grid[robot_y][gap_x] = 'O';
                    grid[robot_y][robot_x] = '.';
                    grid[robot_y][robot_x + 1] = '@';
                    robot_x += 1;
                }
            }
            '^' => {
                let mut gap_y = robot_y;
                while grid[gap_y][robot_x] != '.' && grid[gap_y][robot_x] != '#' {
                    gap_y -= 1;
                }
                if grid[gap_y][robot_x] == '.' {
                    grid[gap_y][robot_x] = 'O';
                    grid[robot_y][robot_x] = '.';
                    grid[robot_y - 1][robot_x] = '@';
                    robot_y -= 1;
                }
            }
            _ => {}
        });

    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'O' {
                total += 100 * y + x;
            }
        }
    }

    println!("{total}")
}
