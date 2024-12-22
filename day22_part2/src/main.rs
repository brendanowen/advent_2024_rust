use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "data.txt";

    let numbers: Vec<usize> = read_numbers(file_path)?;

    let mut total_hashmap: HashMap<[i64; 4], usize> = HashMap::new();
    for i in numbers {
        let mut current: usize = i;
        let mut sequence: Vec<(usize, i64)> = Vec::new();
        for _ in 0..2000 {
            let x1: usize = ((current * 64) ^ current) % 16777216;
            let x2: usize = ((x1 / 32) ^ x1) % 16777216;
            let x3: usize = ((x2 * 2048) ^ x2) % 16777216;

            let last: usize = current % 10;
            let value: usize = x3 % 10;
            let difference: i64 = value as i64 - last as i64;
            current = x3;
            sequence.push((value, difference));
        }
        let mut current_hashset: HashSet<[i64; 4]> = HashSet::new();
        for i in 3..2000 {
            let check: [i64; 4] = [
                sequence[i - 3].1,
                sequence[i - 2].1,
                sequence[i - 1].1,
                sequence[i].1,
            ];
            if current_hashset.contains(&check) {
                continue;
            }
            current_hashset.insert(check);
            *total_hashmap.entry(check).or_insert(0) += sequence[i].0; // Inserts 0 if the key is not present and then adds increment value.
        }
    }

    let total = total_hashmap.iter().max_by_key(|(_, &v)| v).unwrap().1;
    println!("{total}");

    Ok(())
}

fn read_numbers<P>(filename: P) -> io::Result<Vec<usize>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut numbers: Vec<usize> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        match line.parse::<usize>() {
            Ok(number) => numbers.push(number),
            Err(err) => {
                eprintln!("Error parsing line '{}': {}", line, err);
                // Consider returning an error here if you want to stop on parsing errors.
                // For example: return Err(io::Error::new(io::ErrorKind::InvalidData, err));
            }
        }
    }

    Ok(numbers)
}
