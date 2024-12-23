use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("data.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut pairs: Vec<([char; 2], [char; 2])> = Vec::new();
    let mut all: Vec<[char; 2]> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.len() == 5 && line.chars().nth(2) == Some('-') {
            let left: [char; 2] = line[0..2]
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            let right: [char; 2] = line[3..5]
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();

            pairs.push((left, right));
            all.push(left);
            all.push(right);
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }
    all.sort_by(|a, b| {
        // Example: Sort by the first character, then by the second in reverse order
        a[0].cmp(&b[0]).then_with(|| b[1].cmp(&a[1]))
    });

    let mut list: Vec<([char; 2], bool)> = Vec::new();
    let mut dictionary: HashMap<[char; 2], (usize, bool)> = HashMap::new();
    let mut total_computers: usize = 0;
    let number = pairs.len();
    for pair1 in all {
        if !dictionary.contains_key(&pair1) {
            let has_t = pair1[0] == 't';
            dictionary.insert(pair1, (total_computers, has_t));
            list.push((pair1, pair1[0] == 't'));
            total_computers += 1;
        }
    }

    let mut joins: Vec<Vec<bool>> = vec![vec![false; total_computers]; total_computers];
    let mut join_list: Vec<Vec<usize>> = vec![vec![]; total_computers];
    for p1 in 0..number {
        let pair1 = pairs[p1];

        let index1 = dictionary.get(&pair1.0).unwrap().0;
        let index2 = dictionary.get(&pair1.1).unwrap().0;

        if index1 < index2 {
            if !joins[index1][index2] {
                joins[index1][index2] = true;
                join_list[index1].push(index2);
            }
        } else {
            if !joins[index2][index1] {
                joins[index2][index1] = true;
                join_list[index2].push(index1);
            }
        }
    }

    let mut options: Vec<Vec<usize>> = (0..total_computers).map(|value| vec![value]).collect();

    for _ in 1..total_computers {
        let mut next_options: Vec<Vec<usize>> = vec![];
        for option in &options {
            let last = *option.last().unwrap();
            for next in &join_list[last] {
                let joined = option.iter().all(|&check| joins[check][*next]);
                if joined {
                    let mut next_group = option.clone();
                    next_group.push(*next);
                    next_options.push(next_group);
                }
            }
        }
        let reduced_option: Vec<Vec<usize>> = next_options
            .iter()
            .filter_map(|test| {
                if test.iter().any(|&check| list[check].1) {
                    Some(test.clone())
                } else {
                    None
                }
            })
            .collect();
        if reduced_option.is_empty() {
            options[0]
                .iter()
                .for_each(|index| print!("{}{},", list[*index].0[0], list[*index].0[1]));
            println!();
            break;
        }
        options = next_options;
    }

    Ok(())
}
