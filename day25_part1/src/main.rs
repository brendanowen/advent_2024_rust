use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
enum ItemType {
    Key,
    Lock,
    Other,
}

#[derive(Debug, Clone)]
struct Item {
    item_type: ItemType,
    widths: Vec<usize>,
}

fn process_data(filename: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut items: Vec<Item> = Vec::new();
    let mut current_item_lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            if !current_item_lines.is_empty() {
                let item = create_item(&current_item_lines);
                items.push(item);
                current_item_lines.clear();
            }
            continue;
        }

        current_item_lines.push(line.to_string());
    }

    // Process the last item if the file doesn't end with an empty line
    if !current_item_lines.is_empty() {
        let item = create_item(&current_item_lines);
        items.push(item);
    }

    Ok(items)
}

fn create_item(lines: &Vec<String>) -> Item {
    let num_rows = lines.len();
    let num_cols = if num_rows > 0 { lines[0].len() } else { 0 };

    let item_type = if lines[0].chars().all(|c| c == '#') {
        ItemType::Key
    } else if num_rows > 0 && lines[num_rows - 1].chars().all(|c| c == '#') {
        ItemType::Lock
    } else {
        ItemType::Other
    };

    let mut widths: Vec<usize> = Vec::new();
    for j in 0..num_cols {
        let mut count = 0;
        for i in 0..num_rows {
            if lines[i].chars().nth(j).unwrap() == '#' {
                count += 1;
            }
        }
        count -= 1;
        widths.push(count);
    }

    Item { item_type, widths }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let items = process_data("data.txt")?;

    let mut locks: Vec<Item> = vec![];
    let mut keys: Vec<Item> = vec![];
    for item in &items {
        if item.item_type == ItemType::Key {
            keys.push(item.clone());
        } else {
            locks.push(item.clone());
        }
    }

    let mut total = 0;
    for key in &keys {
        for lock in &locks {
            let mut ok: bool = true;
            for i in 0..5 {
                if key.widths[i] + lock.widths[i] > 5 {
                    ok = false;
                }
            }
            if ok {
                total += 1;
            }
        }
    }

    println!("{total}");
    Ok(())
}
