use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
struct Button {
    x_offset: i64,
    y_offset: i64,
}

#[derive(Default)]
struct Prize {
    x_coordinate: i64,
    y_coordinate: i64,
}

struct InputData {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("machines.txt")?;
    let reader = BufReader::new(file);

    let mut input_data: Vec<InputData> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();

        if parts.len() != 2 {
            continue;
        }

        let label = parts[0];
        let values = parts[1].split(", ").collect::<Vec<&str>>();

        match label {
            "Button A" => {
                let (x_offset, y_offset) = parse_coordinates(values[0], values[1])?;
                input_data.push(InputData {
                    button_a: Button { x_offset, y_offset },
                    button_b: Default::default(),
                    prize: Default::default(),
                });
            }
            "Button B" => {
                let (x_offset, y_offset) = parse_coordinates(values[0], values[1])?;
                input_data.last_mut().unwrap().button_b = Button { x_offset, y_offset };
            }
            "Prize" => {
                let (x_coordinate, y_coordinate) = parse_coordinates(values[0], values[1])?;
                input_data.last_mut().unwrap().prize = Prize {
                    x_coordinate: x_coordinate + 10000000000000,
                    y_coordinate: y_coordinate + 10000000000000,
                };
            }
            _ => {}
        }
    }

    let mut total = 0;
    input_data.iter().for_each(|machine| {
        let a = machine.button_a.x_offset;
        let b = machine.button_b.x_offset;
        let c = machine.button_a.y_offset;
        let d = machine.button_b.y_offset;
        let x = machine.prize.x_coordinate;
        let y = machine.prize.y_coordinate;
        let det = a * d - c * b;
        if det != 0 {
            let det_a = d * x - b * y;
            let det_b = -c * x + a * y;
            if det_a % det == 0 && det_b % det == 0 {
                let a_presses = det_a / det;
                let b_presses = det_b / det;

                let cost = 3 * a_presses + b_presses;
                total += cost;
            }
        }
    });

    println!("{total}");

    Ok(())
}

fn parse_coordinates(x_str: &str, y_str: &str) -> Result<(i64, i64), std::io::Error> {
    match (
        x_str
            .strip_prefix("X+")
            .unwrap_or(x_str.strip_prefix("X=").unwrap_or(x_str))
            .parse::<i64>(),
        y_str
            .strip_prefix("Y+")
            .unwrap_or(y_str.strip_prefix("Y=").unwrap_or(y_str))
            .parse::<i64>(),
    ) {
        (Ok(x_offset), Ok(y_offset)) => Ok((x_offset, y_offset)),
        (Err(_), _) | (_, Err(_)) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to parse coordinates",
        )),
    }
}
