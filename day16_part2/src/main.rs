use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("problem.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut s_index = None;
    let mut e_index = None;

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let row: Vec<char> = line.chars().collect();
        grid.push(row.clone());

        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                s_index = Some((x, y));
            } else if *c == 'E' {
                e_index = Some((x, y));
            }
        }
    }

    let width = grid[0].len();
    let height = grid.len();

    let mut min_score: Vec<Vec<Vec<Option<usize>>>> = vec![vec![vec![None; 4]; width]; height];
    let e_x = e_index.unwrap().0;
    let e_y = e_index.unwrap().1;
    let s_x = s_index.unwrap().0;
    let s_y = s_index.unwrap().1;

    let turn_cost = 1000;
    let move_cost = 1;

    min_score[s_y][s_x][0] = Some(0);

    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..4 {
            for x in 1..(width - 1) {
                for y in 1..(height - 1) {
                    if grid[y][x] != '#' {
                        if let Some(cost) = min_score[y][x][i] {
                            let clockwise = (i + 3) % 4;
                            let anticlockwise = (i + 1) % 4;
                            let turned = cost + turn_cost;
                            if let Some(compare) = min_score[y][x][clockwise] {
                                if turned < compare {
                                    min_score[y][x][clockwise] = Some(turned);
                                    changed = true;
                                }
                            } else {
                                min_score[y][x][clockwise] = Some(turned);
                                changed = true;
                            }
                            if let Some(compare) = min_score[y][x][anticlockwise] {
                                if turned < compare {
                                    min_score[y][x][anticlockwise] = Some(turned);
                                    changed = true;
                                }
                            } else {
                                min_score[y][x][anticlockwise] = Some(turned);
                                changed = true;
                            }

                            let moved = cost + move_cost;
                            if i == 0 && grid[y][x + 1] != '#' {
                                if let Some(compare) = min_score[y][x + 1][i] {
                                    if turned < compare {
                                        min_score[y][x + 1][i] = Some(moved);
                                        changed = true;
                                    }
                                } else {
                                    min_score[y][x + 1][i] = Some(moved);
                                    changed = true;
                                }
                            }
                            if i == 1 && grid[y + 1][x] != '#' {
                                if let Some(compare) = min_score[y + 1][x][i] {
                                    if turned < compare {
                                        min_score[y + 1][x][i] = Some(moved);
                                        changed = true;
                                    }
                                } else {
                                    min_score[y + 1][x][i] = Some(moved);
                                    changed = true;
                                }
                            }
                            if i == 2 && grid[y][x - 1] != '#' {
                                if let Some(compare) = min_score[y][x - 1][i] {
                                    if turned < compare {
                                        min_score[y][x - 1][i] = Some(moved);
                                        changed = true;
                                    }
                                } else {
                                    min_score[y][x - 1][i] = Some(moved);
                                    changed = true;
                                }
                            }
                            if i == 3 && grid[y - 1][x] != '#' {
                                if let Some(compare) = min_score[y - 1][x][i] {
                                    if turned < compare {
                                        min_score[y - 1][x][i] = Some(moved);
                                        changed = true;
                                    }
                                } else {
                                    min_score[y - 1][x][i] = Some(moved);
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let values = vec![
        min_score[e_y][e_x][0].unwrap(),
        min_score[e_y][e_x][1].unwrap(),
        min_score[e_y][e_x][2].unwrap(),
        min_score[e_y][e_x][3].unwrap(),
    ];
    let min_value = values.iter().min();

    let mut placed: Vec<Vec<Vec<char>>> = vec![vec![vec!['.'; 4]; width]; height];
    for i in 0..4 {
        if let Some(test_value) = min_score[e_y][e_x][i] {
            if test_value == *min_value.unwrap() {
                placed[e_y][e_x][i] = 'O';
            }
        }
    }
    let mut changed = true;
    while changed {
        changed = false;
        for x in 1..(width - 1) {
            for y in 1..(height - 1) {
                if grid[y][x] != '#' {
                    for i in 0..4 {
                        let clockwise = (i + 3) % 4;
                        let anticlockwise = (i + 1) % 4;

                        let mut turning = false;

                        if let Some(compare) = min_score[y][x][clockwise] {
                            if let Some(cost) = min_score[y][x][i] {
                                if compare == cost + turn_cost
                                    && placed[y][x][clockwise] == 'O'
                                    && placed[y][x][i] != 'O'
                                {
                                    turning = true;
                                }
                            }
                        }
                        if let Some(compare) = min_score[y][x][anticlockwise] {
                            if let Some(cost) = min_score[y][x][i] {
                                if compare == cost + turn_cost
                                    && placed[y][x][anticlockwise] == 'O'
                                    && placed[y][x][i] != 'O'
                                {
                                    turning = true;
                                }
                            }
                        }
                        if turning {
                            placed[y][x][i] = 'O';
                            changed = true;
                        }
                    }

                    if let Some(compare) = min_score[y][x + 1][0] {
                        if let Some(cost) = min_score[y][x][0] {
                            if compare == cost + move_cost
                                && placed[y][x + 1][0] == 'O'
                                && placed[y][x][0] != 'O'
                            {
                                placed[y][x][0] = 'O';
                                changed = true;
                            }
                        }
                    }
                    if let Some(compare) = min_score[y + 1][x][1] {
                        if let Some(cost) = min_score[y][x][1] {
                            if compare == cost + move_cost
                                && placed[y + 1][x][1] == 'O'
                                && placed[y][x][1] != 'O'
                            {
                                placed[y][x][1] = 'O';
                                changed = true;
                            }
                        }
                    }
                    if let Some(compare) = min_score[y][x - 1][2] {
                        if let Some(cost) = min_score[y][x][2] {
                            if compare == cost + move_cost
                                && placed[y][x - 1][2] == 'O'
                                && placed[y][x][2] != 'O'
                            {
                                placed[y][x][2] = 'O';
                                changed = true;
                            }
                        }
                    }
                    if let Some(compare) = min_score[y - 1][x][3] {
                        if let Some(cost) = min_score[y][x][3] {
                            if compare == cost + move_cost
                                && placed[y - 1][x][3] == 'O'
                                && placed[y][x][3] != 'O'
                            {
                                placed[y][x][3] = 'O';
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut place_length = 0;
    for y in 0..height {
        for x in 0..width {
            if placed[y][x].iter().any(|test| *test == 'O') {
                place_length += 1;
            }
        }
    }
    println!("{}", place_length);
}
