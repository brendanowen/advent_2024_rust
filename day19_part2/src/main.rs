use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() -> io::Result<()> {
    let path = Path::new("data.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut towels: Vec<String> = Vec::new();
    let mut designs: Vec<String> = Vec::new();
    let mut reading_first_row = true;

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            if reading_first_row {
                reading_first_row = false;
            }
            continue; // Skip empty lines
        }

        if reading_first_row {
            let parts: Vec<&str> = trimmed_line.split(',').map(|s| s.trim()).collect();
            towels = parts.iter().map(|s| s.replace(" ", "")).collect();
        } else {
            designs.push(trimmed_line.to_string());
        }
    }

    let total: usize = designs
        .iter()
        .map(|current| {
            let length = current.len();
            let mut filled: Vec<usize> = vec![0; length + 1];
            filled[length] = 1;
            for i in (0..length).rev() {
                let mut found = 0;
                for towel in towels.clone() {
                    let end = i + towel.len();
                    if end <= length {
                        if current[i..end] == towel {
                            found += filled[end];
                        }
                    }
                }
                filled[i] = found;
            }
            filled[0]
        })
        .sum();

    println!("{total}");

    Ok(())
}
