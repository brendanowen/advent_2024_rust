use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "example.txt";

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
            println!("{path_length}");
            instruction.1 * path_length
        })
        .sum();

    println!("{}", total);

    Ok(())
}

fn print_chars(chars: &Vec<char>) {
    chars.iter().for_each(|char_value| print!("{}", char_value));
    println!();
}

fn measure_length(char_vec: &Vec<char>, numerical_robots: usize) -> usize {
    print_chars(char_vec);
    let mut path: Vec<char> = generate_numerical_path(char_vec);
    print_chars(&path);
    for _ in 0..numerical_robots {
        path = generate_direction_path(&path);
        print_chars(&path);
    }

    path.len()
}

fn generate_numerical_path(char_vec: &Vec<char>) -> Vec<char> {
    let mut return_path: Vec<char> = Vec::new();
    let mut last: char = 'A';
    char_vec.iter().for_each(|char_value| {
        let new_string: &str = match last {
            'A' => match char_value {
                'A' => "A",
                '0' => "<A",
                '1' => "^<<A",
                '2' => "^<A",
                '3' => "^A",
                '4' => "^^<<A",
                '5' => "^^<A",
                '6' => "^^A",
                '7' => "^^^<<A",
                '8' => "^^^<A",
                '9' => "^^^A",
                _ => "",
            },
            '0' => match char_value {
                'A' => ">A",
                '0' => "A",
                '1' => "^<A",
                '2' => "^A",
                '3' => "^>A",
                '4' => "^^<A",
                '5' => "^^A",
                '6' => "^^>A",
                '7' => "^^^<A",
                '8' => "^^^A",
                '9' => "^^^>A",
                _ => "",
            },
            '1' => match char_value {
                'A' => ">>vA",
                '0' => ">vA",
                '1' => "A",
                '2' => ">A",
                '3' => ">>A",
                '4' => "^A",
                '5' => "^>A",
                '6' => "^>>A",
                '7' => "^^A",
                '8' => "^^>A",
                '9' => "^^>>A",
                _ => "",
            },
            '2' => match char_value {
                'A' => ">vA",
                '0' => "vA",
                '1' => "<A",
                '2' => "A",
                '3' => ">A",
                '4' => "^<A",
                '5' => "^A",
                '6' => "^>A",
                '7' => "^^<A",
                '8' => "^^A",
                '9' => "^^>A",
                _ => "",
            },
            '3' => match char_value {
                'A' => "vA",
                '0' => "v<A",
                '1' => "<<A",
                '2' => "<A",
                '3' => "A",
                '4' => "^<<A",
                '5' => "^<A",
                '6' => "^A",
                '7' => "^^<<A",
                '8' => "^^<A",
                '9' => "^^A",
                _ => "",
            },
            '4' => match char_value {
                'A' => ">>vvA",
                '0' => ">vvA",
                '1' => "vA",
                '2' => ">vA",
                '3' => ">>vA",
                '4' => "A",
                '5' => ">A",
                '6' => ">>A",
                '7' => "^A",
                '8' => "^>A",
                '9' => "^>>A",
                _ => "",
            },
            '5' => match char_value {
                'A' => ">vvA",
                '0' => "vvA",
                '1' => "v<A",
                '2' => "vA",
                '3' => ">vA",
                '4' => "<A",
                '5' => "A",
                '6' => ">A",
                '7' => "^<A",
                '8' => "^A",
                '9' => "^>A",
                _ => "",
            },
            '6' => match char_value {
                'A' => "vvA",
                '0' => "vv<A",
                '1' => "v<<A",
                '2' => "v<A",
                '3' => "vA",
                '4' => "<<A",
                '5' => "<A",
                '6' => "A",
                '7' => "^<<A",
                '8' => "^<A",
                '9' => "^A",
                _ => "",
            },
            '7' => match char_value {
                'A' => ">>vvvA",
                '0' => ">vvvA",
                '1' => "vvA",
                '2' => ">vvA",
                '3' => ">>vvA",
                '4' => "vA",
                '5' => "v>A",
                '6' => "v>>A",
                '7' => "A",
                '8' => ">A",
                '9' => ">>A",
                _ => "",
            },
            '8' => match char_value {
                'A' => ">vvvA",
                '0' => "vvvA",
                '1' => "vv<A",
                '2' => "vvA",
                '3' => ">vvA",
                '4' => "v<A",
                '5' => "vA",
                '6' => ">vA",
                '7' => "<A",
                '8' => "A",
                '9' => ">A",
                _ => "",
            },
            '9' => match char_value {
                'A' => "vvvA",
                '0' => "vvv<A",
                '1' => "vv<<A",
                '2' => "vv<A",
                '3' => "vvA",
                '4' => "v<<A",
                '5' => "v<A",
                '6' => "vA",
                '7' => "<<A",
                '8' => "<A",
                '9' => "A",
                _ => "",
            },
            _ => "",
        };
        let new_sequence: Vec<char> = new_string.chars().collect();
        return_path.extend(new_sequence);

        last = *char_value
    });

    return_path
}
fn generate_direction_path(char_vec: &Vec<char>) -> Vec<char> {
    let mut return_path: Vec<char> = Vec::new();
    let mut last: char = 'A';
    char_vec.iter().for_each(|char_value| {
        let new_string: &str = match last {
            'A' => match char_value {
                'A' => "A",
                '^' => "<A",
                '<' => "v<<A",
                'v' => "v<A",
                '>' => "vA",
                _ => "",
            },
            '^' => match char_value {
                'A' => ">A",
                '^' => "A",
                '<' => "v<A",
                'v' => "vA",
                '>' => ">vA",
                _ => "",
            },
            '<' => match char_value {
                'A' => ">>^A",
                '^' => "^>A",
                '<' => "A",
                'v' => ">A",
                '>' => ">>A",
                _ => "",
            },
            'v' => match char_value {
                'A' => ">^A",
                '^' => "^A",
                '<' => "<A",
                'v' => "A",
                '>' => ">A",
                _ => "",
            },
            '>' => match char_value {
                'A' => "^A",
                '^' => "^<A",
                '<' => "<<A",
                'v' => "<A",
                '>' => "A",
                _ => "",
            },
            _ => "",
        };
        let new_sequence: Vec<char> = new_string.chars().collect();
        return_path.extend(new_sequence);

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
