use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "data.txt";

    // Read the file line by line
    let lines = read_lines(file_path)?;

    // Create a Vec<Vec<char>> to store the characters of each line
    let mut instructions: Vec<(Vec<char>, usize)> = Vec::new();

    for line in lines {
        let line = line?; // Handle potential errors reading a line
        let char_vec: Vec<char> = line.chars().collect();
        match convert_to_numerical(&line) {
            Ok(value) => {
                instructions.push((char_vec, value));
            }
            Err(err) => eprintln!("Error converting line '{}': {}", line, err), // Handle errors
        }
    }

    let total: usize = instructions
        .iter()
        .map(|instruction| {
            //
            let path_length = measure_length(&instruction.0, 2);
            instruction.1 * path_length
        })
        .sum();

    println!("{}", total);

    Ok(())
}

fn measure_length(char_vec: &Vec<char>, numerical_robots: usize) -> usize {
    let mut paths: Vec<Vec<char>> = generate_numerical_paths(char_vec);

    for _ in 0..numerical_robots {
        let mut next_paths: Vec<Vec<char>> = vec![];
        paths.iter().for_each(|current_path| {
            let new_list = generate_direction_path(&current_path);
            next_paths.extend(new_list);
        });
        paths = next_paths;
    }

    let mut min_length = paths[0].len();
    paths.iter().for_each(|current_path| {
        if current_path.len() < min_length {
            min_length = current_path.len();
        }
    });

    min_length
}

fn commands_robot(last: char, char_value: char) -> Vec<String> {
    let new_string: Vec<&str> = match last {
        'A' => match char_value {
            'A' => vec!["A"],
            '^' => vec!["<A"],
            '<' => vec!["v<<A", "<v<A"],
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
            'A' => vec![">>^A", ">^>A"],
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
            '1' => vec!["^<<A", "<^<A"],
            '2' => vec!["^<A", "<^A"],
            '3' => vec!["^A"],
            '4' => vec!["^^<<A", "^<<^A", "<^^<A"],
            '5' => vec!["^^<A", "<^^A"],
            '6' => vec!["^^A"],
            '7' => vec!["^^^<<A", "<^^^<A", "^<<^^A", "^^<<^A"],
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
            '4' => vec!["^^<A", "^<^A"],
            '5' => vec!["^^A"],
            '6' => vec!["^^>A", ">^^A"],
            '7' => vec!["^^^<A", "^^<^A", "^<^^A"],
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
            '8' => vec!["^^>A", "^>^A", ">^^A"],
            '9' => vec!["^^>>A", "^>^>A", ">>^^A", "^>>^A", ">^^>A", ">^>^A"],
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
            '7' => vec!["^^<A", "^<^A", "<^^A"],
            '8' => vec!["^^A"],
            '9' => vec!["^^>A", "^>^A", ">^^A"],
            _ => vec![""],
        },
        '3' => match char_value {
            'A' => vec!["vA"],
            '0' => vec!["v<A", "<vA"],
            '1' => vec!["<<A"],
            '2' => vec!["<A"],
            '3' => vec!["A"],
            '4' => vec!["^<<A", "<^<A", "<<^A"],
            '5' => vec!["^<A", "<^A"],
            '6' => vec!["^A"],
            '7' => vec!["<<^^A", "<^<^A", "^<<^A", "<^^<A", "^^<<A", "^<^<^A"],
            '8' => vec!["^^<A", "^<^A", "<^^A"],
            '9' => vec!["^^A"],
            _ => vec![""],
        },
        '4' => match char_value {
            'A' => vec![">>vvA", ">v>vA", ">vv>A"],
            '0' => vec![">vvA", "v>vA"],
            '1' => vec!["vA"],
            '2' => vec![">vA", "v>A"],
            '3' => vec![">>vA", ">v>A", "v>>A"],
            '4' => vec!["A"],
            '5' => vec![">A"],
            '6' => vec![">>A"],
            '7' => vec!["^A"],
            '8' => vec!["^>A", ">^A"],
            '9' => vec!["^>>A", ">^>A", ">>^A"],
            _ => vec![""],
        },
        '5' => match char_value {
            'A' => vec![">vvA", "v>vA", "vv>A"],
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
            '0' => vec!["vv<A", "v<vA", "<vvA"],
            '1' => vec!["v<<A", "<v<A", "<<vA"],
            '2' => vec!["v<A", "<vA"],
            '3' => vec!["vA"],
            '4' => vec!["<<A"],
            '5' => vec!["<A"],
            '6' => vec!["A"],
            '7' => vec!["^<<A", "<^<A", "<<^A"],
            '8' => vec!["^<A", "<^A"],
            '9' => vec!["^A"],
            _ => vec![""],
        },
        '7' => match char_value {
            'A' => vec![">>vvvA", ">vvv>A", ">vv>vA", ">v>vv", "vv>v>A", "vv>>vA"],
            '0' => vec![">vvvA", "v>vvA", "vv>vA"],
            '1' => vec!["vvA"],
            '2' => vec![">vvA", "v>vA", "vv>A"],
            '3' => vec![">>vvA", ">v>vA", ">vv>A", "v>>vA", "v>v>vA", "vv>>A"],
            '4' => vec!["vA"],
            '5' => vec!["v>A", ">vA"],
            '6' => vec!["v>>A", ">v>A", ">>vA"],
            '7' => vec!["A"],
            '8' => vec![">A"],
            '9' => vec![">>A"],
            _ => vec![""],
        },
        '8' => match char_value {
            'A' => vec![">vvvA", "v>vvA", "vv>vA", "vvv>A"],
            '0' => vec!["vvvA"],
            '1' => vec!["vv<A", "v<vA", "<vvA"],
            '2' => vec!["vvA"],
            '3' => vec![">vvA", "v>vA", "vv>A"],
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
            '0' => vec!["vvv<A", "vv<vA", "v<vvA", "<vvvA"],
            '1' => vec!["vv<<A", "v<<vA", "v<v<A", "<vv<A", "<v<vA", "<<vvA"],
            '2' => vec!["vv<A", "v<VA", "<vvA"],
            '3' => vec!["vvA"],
            '4' => vec!["v<<A", "<V<A", "<<VA"],
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

fn generate_numerical_paths(char_vec: &Vec<char>) -> Vec<Vec<char>> {
    let mut return_path: Vec<Vec<char>> = vec![vec![]];
    let mut last: char = 'A';
    char_vec.iter().for_each(|char_value| {
        let new_strings: Vec<String> = commands_numerical(last, *char_value);
        let mut next_paths: Vec<Vec<char>> = vec![];
        return_path.iter().for_each(|base| {
            new_strings.iter().for_each(|extra| {
                let new_sequence: Vec<char> = extra.chars().collect();
                let mut final_sequence = base.clone();
                final_sequence.extend(new_sequence);
                next_paths.push(final_sequence);
            });
        });
        return_path = next_paths;

        last = *char_value
    });

    return_path
}

fn generate_direction_path(char_vec: &Vec<char>) -> Vec<Vec<char>> {
    let mut return_path: Vec<Vec<char>> = vec![vec![]];
    let mut last: char = 'A';
    char_vec.iter().for_each(|char_value| {
        let new_strings: Vec<String> = commands_robot(last, *char_value);
        let mut next_paths: Vec<Vec<char>> = vec![];
        return_path.iter().for_each(|base| {
            new_strings.iter().for_each(|extra| {
                let new_sequence: Vec<char> = extra.chars().collect();
                let mut final_sequence = base.clone();
                final_sequence.extend(new_sequence);
                next_paths.push(final_sequence);
            });
        });
        return_path = next_paths;

        last = *char_value
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
