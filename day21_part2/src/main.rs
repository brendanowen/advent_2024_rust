use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "data.txt";

    // Read the file line by line
    let lines = read_lines(file_path)?;

    // Create a Vec<Vec<char>> to store the characters of each line
    let mut instructions: Vec<(Vec<usize>, usize)> = Vec::new();

    let digits: Vec<char> = vec!['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let controls: Vec<char> = vec!['A', '^', '>', 'v', '<'];

    // Create HashMaps for efficient lookup
    let digits_map: HashMap<char, usize> =
        digits.iter().enumerate().map(|(i, &c)| (c, i)).collect();
    let controls_map: HashMap<char, usize> =
        controls.iter().enumerate().map(|(i, &c)| (c, i)).collect();

    for line in lines {
        let line = line?; // Handle potential errors reading a line
        let char_vec: Vec<char> = line.chars().collect();
        match convert_to_numerical(&line) {
            Ok(value) => {
                let indexes: Vec<usize> = char_vec
                    .iter()
                    .map(|character| *digits_map.get(&character).unwrap())
                    .collect();
                instructions.push((indexes, value));
            }
            Err(err) => eprintln!("Error converting line '{}': {}", line, err), // Handle errors
        }
    }

    let path_controls2: Vec<Vec<Vec<usize>>> = controls
        .iter()
        .map(|first| {
            controls
                .iter()
                .map(|second| {
                    let best_path = find_best_robot_path2(first, second);
                    let index_list: Vec<usize> = best_path[0]
                        .chars()
                        .map(|character| *controls_map.get(&character).unwrap())
                        .collect();
                    index_list
                })
                .collect()
        })
        .collect();

    println!();
    let path_controls3: Vec<Vec<Vec<usize>>> = controls
        .iter()
        .map(|first| {
            controls
                .iter()
                .map(|second| {
                    let best_path = find_best_robot_path3(first, second);
                    let index_list: Vec<usize> = best_path[0]
                        .chars()
                        .map(|character| *controls_map.get(&character).unwrap())
                        .collect();
                    index_list
                })
                .collect()
        })
        .collect();

    println!();

    let path_digits: Vec<Vec<Vec<usize>>> = digits
        .iter()
        .map(|first| {
            digits
                .iter()
                .map(|second| {
                    let best_path = find_best_digit_path4(first, second);
                    let index_list: Vec<usize> = best_path[0]
                        .chars()
                        .map(|character| *controls_map.get(&character).unwrap())
                        .collect();
                    index_list
                })
                .collect()
        })
        .collect();

    println!();

    let mut total: usize = 0;
    for instruction in instructions {
        let mut last_digit: usize = 0;
        let mut current_string: Vec<usize> = vec![];
        for current_digit in instruction.0 {
            let extra_string = path_digits[last_digit][current_digit].clone();
            current_string.extend(extra_string);
            last_digit = current_digit;
        }

        let mut counts: HashMap<(usize, usize), usize> = HashMap::new();

        last_digit = 0;
        for current_digit in current_string {
            *counts.entry((last_digit, current_digit)).or_insert(0) += 1;
            last_digit = current_digit;
        }

        for _ in 0..25 {
            let mut next_counts: HashMap<(usize, usize), usize> = HashMap::new();
            for ((first, second), count) in counts {
                let extra_string = path_controls3[first][second].clone();
                last_digit = 0;
                for current_digit in extra_string {
                    *next_counts.entry((last_digit, current_digit)).or_insert(0) += count;
                    last_digit = current_digit;
                }
            }
            counts = next_counts;
        }

        let sub_total: usize = counts.iter().map(|((_, _), count)| *count).sum();
        total += sub_total * instruction.1;
    }

    println!("{total}");
    Ok(())
}

fn find_best_robot_path4(first: &char, second: &char) -> Vec<String> {
    let paths1: Vec<String> = commands_robot(*first, *second);
    let mut min_length = 100000000000;
    let mut return_string: Vec<String> = vec![];
    paths1.iter().for_each(|path1| {
        let paths2: Vec<String> = generate_direction_path(path1);
        let mut current_min = 100000000000;
        paths2.iter().for_each(|path2| {
            let paths3: Vec<String> = generate_direction_path(path2);
            paths3.iter().for_each(|path3| {
                let paths4: Vec<String> = generate_direction_path(path3);
                for check in paths4 {
                    if check.len() < current_min {
                        current_min = check.len();
                    }
                }
            });
        });
        if current_min < min_length {
            min_length = current_min;
            return_string = vec![path1.clone()];
        } else if current_min == min_length {
            return_string.push(path1.clone());
        }
    });

    println!("{} {} = {}", first, second, return_string.len());

    return_string
}

fn find_best_digit_path4(first: &char, second: &char) -> Vec<String> {
    let paths1: Vec<String> = commands_numerical(*first, *second);
    let mut min_length = 100000000000;
    let mut return_string: Vec<String> = vec![];
    paths1.iter().for_each(|path1| {
        let paths2: Vec<String> = generate_direction_path(path1);
        let mut current_min = 100000000000;
        paths2.iter().for_each(|path2| {
            let paths3: Vec<String> = generate_direction_path(path2);
            paths3.iter().for_each(|path3| {
                let paths4: Vec<String> = generate_direction_path(path3);
                for check in paths4 {
                    if check.len() < current_min {
                        current_min = check.len();
                    }
                }
            });
        });
        if current_min < min_length {
            min_length = current_min;
            return_string = vec![path1.clone()];
        } else if current_min == min_length {
            return_string.push(path1.clone());
        }
    });

    println!("{} {} = {}", first, second, return_string.len());
    return_string
}

fn find_best_robot_path3(first: &char, second: &char) -> Vec<String> {
    let paths1: Vec<String> = commands_robot(*first, *second);
    let mut min_length = 100000000000;
    let mut return_string: Vec<String> = vec![];
    paths1.iter().for_each(|path1| {
        let paths2: Vec<String> = generate_direction_path(path1);
        let mut current_min = 100000000000;
        paths2.iter().for_each(|path2| {
            let paths3: Vec<String> = generate_direction_path(path2);
            for check in paths3 {
                if check.len() < current_min {
                    current_min = check.len();
                }
            }
        });
        if current_min < min_length {
            min_length = current_min;
            return_string = vec![path1.clone()];
        } else if current_min == min_length {
            return_string.push(path1.clone());
        }
    });

    println!("{} {} = {}", first, second, return_string.len());

    return_string
}

fn find_best_digit_path3(first: &char, second: &char) -> Vec<String> {
    let paths1: Vec<String> = commands_numerical(*first, *second);
    let mut min_length = 100000000000;
    let mut return_string: Vec<String> = vec![];
    paths1.iter().for_each(|path1| {
        let paths2: Vec<String> = generate_direction_path(path1);
        let mut current_min = 100000000000;
        paths2.iter().for_each(|path2| {
            let paths3: Vec<String> = generate_direction_path(path2);
            for check in paths3 {
                if check.len() < current_min {
                    current_min = check.len();
                }
            }
        });
        if current_min < min_length {
            min_length = current_min;
            return_string = vec![path1.clone()];
        } else if current_min == min_length {
            return_string.push(path1.clone());
        }
    });

    println!("{} {} = {}", first, second, return_string.len());
    return_string
}

fn find_best_robot_path2(first: &char, second: &char) -> Vec<String> {
    let paths1: Vec<String> = commands_robot(*first, *second);
    let mut min_length = 100000000000;
    let mut return_string: Vec<String> = vec![];
    paths1.iter().for_each(|path1| {
        let paths2: Vec<String> = generate_direction_path(path1);
        let mut current_min = 100000000000;
        for check in paths2 {
            if check.len() < current_min {
                current_min = check.len();
            }
        }
        if current_min < min_length {
            min_length = current_min;
            return_string = vec![path1.clone()];
        } else if current_min == min_length {
            return_string.push(path1.clone());
        }
    });

    println!("{} {} = {}", first, second, return_string.len());

    return_string
}

fn find_best_digit_path2(first: &char, second: &char) -> Vec<String> {
    let paths1: Vec<String> = commands_numerical(*first, *second);
    let mut min_length = 100000000000;
    let mut return_string: Vec<String> = vec![];
    paths1.iter().for_each(|path1| {
        let paths2: Vec<String> = generate_direction_path(path1);
        let mut current_min = 100000000000;
        for check in paths2 {
            if check.len() < current_min {
                current_min = check.len();
            }
        }
        if current_min < min_length {
            min_length = current_min;
            return_string = vec![path1.clone()];
        } else if current_min == min_length {
            return_string.push(path1.clone());
        }
    });

    println!("{} {} = {}", first, second, return_string.len());
    return_string
}

fn commands_robot(last: char, char_value: char) -> Vec<String> {
    let new_string: Vec<&str> = match last {
        'A' => match char_value {
            'A' => vec!["A"],
            '^' => vec!["<A"],
            '<' => vec!["v<<A"],
            'v' => vec!["v<A", "<vA"],
            '>' => vec!["vA"],
            _ => vec![""],
        },
        '^' => match char_value {
            'A' => vec![">A"],
            '^' => vec!["A"],
            '<' => vec!["v<A"],
            'v' => vec!["vA"],
            '>' => vec![">vA", "v>A"],
            _ => vec![""],
        },
        '<' => match char_value {
            'A' => vec![">>^A"],
            '^' => vec![">^A"],
            '<' => vec!["A"],
            'v' => vec![">A"],
            '>' => vec![">>A"],
            _ => vec![""],
        },
        'v' => match char_value {
            'A' => vec![">^A", "^>A"],
            '^' => vec!["^A"],
            '<' => vec!["<A"],
            'v' => vec!["A"],
            '>' => vec![">A"],
            _ => vec![""],
        },
        '>' => match char_value {
            'A' => vec!["^A"],
            '^' => vec!["^<A", "<^A"],
            '<' => vec!["<<A"],
            'v' => vec!["<A"],
            '>' => vec!["A"],
            _ => vec![""],
        },
        _ => vec![""],
    };
    new_string.iter().map(|string| string.to_string()).collect()
}

fn commands_numerical(last: char, char_value: char) -> Vec<String> {
    let new_string: Vec<&str> = match last {
        'A' => match char_value {
            'A' => vec!["A"],
            '0' => vec!["<A"],
            '1' => vec!["^<<A"],
            '2' => vec!["^<A", "<^A"],
            '3' => vec!["^A"],
            '4' => vec!["^^<<A"],
            '5' => vec!["^^<A", "<^^A"],
            '6' => vec!["^^A"],
            '7' => vec!["^^^<<A"],
            '8' => vec!["^^^<A", "<^^^A"],
            '9' => vec!["^^^A"],
            _ => vec![""],
        },
        '0' => match char_value {
            'A' => vec![">A"],
            '0' => vec!["A"],
            '1' => vec!["^<A"],
            '2' => vec!["^A"],
            '3' => vec!["^>A", ">^A"],
            '4' => vec!["^^<A"],
            '5' => vec!["^^A"],
            '6' => vec!["^^>A", ">^^A"],
            '7' => vec!["^^^<A"],
            '8' => vec!["^^^A"],
            '9' => vec!["^^^>A", ">^^^A"],
            _ => vec![""],
        },
        '1' => match char_value {
            'A' => vec![">>vA", ">v>A"],
            '0' => vec![">vA"],
            '1' => vec!["A"],
            '2' => vec![">A"],
            '3' => vec![">>A"],
            '4' => vec!["^A"],
            '5' => vec!["^>A", ">^A"],
            '6' => vec!["^>>A", ">>^A"],
            '7' => vec!["^^A"],
            '8' => vec!["^^>A", ">^^A"],
            '9' => vec!["^^>>A", ">>^^A"],
            _ => vec![""],
        },
        '2' => match char_value {
            'A' => vec![">vA", "v>A"],
            '0' => vec!["vA"],
            '1' => vec!["<A"],
            '2' => vec!["A"],
            '3' => vec![">A"],
            '4' => vec!["^<A", "<^A"],
            '5' => vec!["^A"],
            '6' => vec!["^>A", ">^A"],
            '7' => vec!["^^<A", "<^^A"],
            '8' => vec!["^^A"],
            '9' => vec!["^^>A", ">^^A"],
            _ => vec![""],
        },
        '3' => match char_value {
            'A' => vec!["vA"],
            '0' => vec!["v<A", "<vA"],
            '1' => vec!["<<A"],
            '2' => vec!["<A"],
            '3' => vec!["A"],
            '4' => vec!["^<<A", "<<^A"],
            '5' => vec!["^<A", "<^A"],
            '6' => vec!["^A"],
            '7' => vec!["<<^^A", "^^<<A"],
            '8' => vec!["^^<A", "<^^A"],
            '9' => vec!["^^A"],
            _ => vec![""],
        },
        '4' => match char_value {
            'A' => vec![">>vvA"],
            '0' => vec![">vvA"],
            '1' => vec!["vA"],
            '2' => vec![">vA", "v>A"],
            '3' => vec![">>vA", "v>>A"],
            '4' => vec!["A"],
            '5' => vec![">A"],
            '6' => vec![">>A"],
            '7' => vec!["^A"],
            '8' => vec!["^>A", ">^A"],
            '9' => vec!["^>>A", ">>^A"],
            _ => vec![""],
        },
        '5' => match char_value {
            'A' => vec![">vvA", "vv>A"],
            '0' => vec!["vvA"],
            '1' => vec!["v<A", "<vA"],
            '2' => vec!["vA"],
            '3' => vec![">vA", "v>A"],
            '4' => vec!["<A"],
            '5' => vec!["A"],
            '6' => vec![">A"],
            '7' => vec!["^<A", "<^A"],
            '8' => vec!["^A"],
            '9' => vec!["^>A", ">^A"],
            _ => vec![""],
        },
        '6' => match char_value {
            'A' => vec!["vvA"],
            '0' => vec!["vv<A", "<vvA"],
            '1' => vec!["v<<A", "<<vA"],
            '2' => vec!["v<A", "<vA"],
            '3' => vec!["vA"],
            '4' => vec!["<<A"],
            '5' => vec!["<A"],
            '6' => vec!["A"],
            '7' => vec!["^<<A", "<<^A"],
            '8' => vec!["^<A", "<^A"],
            '9' => vec!["^A"],
            _ => vec![""],
        },
        '7' => match char_value {
            'A' => vec![">>vvvA", ">vvv>A"],
            '0' => vec![">vvvA"],
            '1' => vec!["vvA"],
            '2' => vec![">vvA", "vv>A"],
            '3' => vec![">>vvA", "vv>>A"],
            '4' => vec!["vA"],
            '5' => vec!["v>A", ">vA"],
            '6' => vec!["v>>A", ">>vA"],
            '7' => vec!["A"],
            '8' => vec![">A"],
            '9' => vec![">>A"],
            _ => vec![""],
        },
        '8' => match char_value {
            'A' => vec![">vvvA", "vvv>A"],
            '0' => vec!["vvvA"],
            '1' => vec!["vv<A", "<vvA"],
            '2' => vec!["vvA"],
            '3' => vec![">vvA", "vv>A"],
            '4' => vec!["v<A", "<vA"],
            '5' => vec!["vA"],
            '6' => vec![">vA", "v>A"],
            '7' => vec!["<A"],
            '8' => vec!["A"],
            '9' => vec![">A"],
            _ => vec![""],
        },
        '9' => match char_value {
            'A' => vec!["vvvA"],
            '0' => vec!["vvv<A", "<vvvA"],
            '1' => vec!["vv<<A", "<<vvA"],
            '2' => vec!["vv<A", "<vvA"],
            '3' => vec!["vvA"],
            '4' => vec!["v<<A", "<<vA"],
            '5' => vec!["v<A", "<vA"],
            '6' => vec!["vA"],
            '7' => vec!["<<A"],
            '8' => vec!["<A"],
            '9' => vec!["A"],
            _ => vec![""],
        },
        _ => vec![""],
    };
    new_string.iter().map(|string| string.to_string()).collect()
}

fn generate_direction_path(char_vec: &String) -> Vec<String> {
    let mut return_path: Vec<String> = vec!["".to_string()];
    let mut last: char = 'A';
    char_vec.chars().for_each(|char_value| {
        let new_strings: Vec<String> = commands_robot(last, char_value);
        let mut next_paths: Vec<String> = vec![];
        return_path.iter().for_each(|base| {
            new_strings.iter().for_each(|extra| {
                let final_sequence = format!("{}{}", base, extra);
                next_paths.push(final_sequence);
            });
        });
        return_path = next_paths;

        last = char_value
    });

    return_path
}

fn convert_to_numerical(line: &str) -> Result<usize, String> {
    let mut numerical_string = String::new();
    for c in line.chars() {
        if c != 'A' {
            numerical_string.push(c);
        }
    }

    if numerical_string.is_empty() {
        return Err("No numerical characters found in line".to_string());
    }

    match numerical_string.parse::<usize>() {
        Ok(value) => Ok(value),
        Err(err) => Err(format!("Failed to parse numerical value: {}", err)),
    }
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
