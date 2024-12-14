use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let lines = read_lines("placement.txt")?;

    let mut data: Vec<(i64, i64, i64, i64)> = Vec::new();

    for line in lines {
        let line = line?;
        if line.is_empty() {
            continue; // Skip empty lines
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Invalid line format: {}", line);
            continue; // Skip invalid lines
        }

        let p_str = parts[0].strip_prefix("p=").unwrap();
        let v_str = parts[1].strip_prefix("v=").unwrap();

        let p_coords: Vec<&str> = p_str.split(',').collect();
        let v_coords: Vec<&str> = v_str.split(',').collect();

        if p_coords.len() != 2 || v_coords.len() != 2 {
            eprintln!("Invalid coordinate format: {}", line);
            continue; // Skip invalid lines
        }

        let px = p_coords[0].parse::<i64>().unwrap();
        let py = p_coords[1].parse::<i64>().unwrap();
        let vx = v_coords[0].parse::<i64>().unwrap();
        let vy = v_coords[1].parse::<i64>().unwrap();

        data.push((px, py, vx, vy));
    }

    let width = 101;
    let height = 103;
    let half_width = width / 2;
    let half_height = height / 2;
    let seconds = 100;

    let mut counts: Vec<usize> = vec![0; 4];

    data.iter().for_each(|&(x, y, vx, vy)| {
        let final_x = ((x + vx * seconds) % width + width) % width;
        let final_y = ((y + vy * seconds) % height + height) % height;

        if final_x < half_width {
            if final_y < half_height {
                counts[0] += 1;
            } else if final_y > half_height {
                counts[1] += 1;
            }
        } else if final_x > half_width {
            if final_y < half_height {
                counts[2] += 1;
            } else if final_y > half_height {
                counts[3] += 1;
            }
        }
    });

    let total = counts[0] * counts[1] * counts[2] * counts[3];

    println!("{total}");

    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
