use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Rule {
    input1: [char; 3],
    operator: Operator,
    input2: [char; 3],
    output: [char; 3],
}

fn parse_file(
    filename: &str,
) -> Result<(HashMap<[char; 3], Option<bool>>, Vec<Rule>), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut values: HashMap<[char; 3], Option<bool>> = HashMap::new();
    let mut rules: Vec<Rule> = Vec::new();

    let mut parsing_values = true;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            parsing_values = false;
            continue;
        }

        if parsing_values {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
                return Err("Invalid value line format".into());
            }
            let key_str = parts[0];
            let value_str = parts[1];

            if key_str.len() != 3 {
                return Err("Invalid key format".into());
            }

            let key: [char; 3] = key_str.chars().collect::<Vec<char>>().try_into().unwrap();

            let value = value_str.parse::<u32>()? == 1;

            values.insert(key, Some(value));
        } else {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts.len() != 5 {
                return Err("Invalid rule line format".into());
            }

            let input1_str = parts[0];
            let operator_str = parts[1];
            let input2_str = parts[2];
            let _arrow = parts[3];
            let output_str = parts[4];

            if input1_str.len() != 3 || input2_str.len() != 3 || output_str.len() != 3 {
                return Err("Invalid key format in rule".into());
            }

            let input1: [char; 3] = input1_str
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            let input2: [char; 3] = input2_str
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            let output: [char; 3] = output_str
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();

            let operator = match operator_str {
                "AND" => Operator::AND,
                "OR" => Operator::OR,
                "XOR" => Operator::XOR,
                _ => return Err("Invalid operator".into()),
            };

            rules.push(Rule {
                input1,
                operator,
                input2,
                output,
            });
        }
    }

    Ok((values, rules))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut values, rules) = parse_file("data.txt")?;

    for rule in &rules {
        if !values.contains_key(&rule.input1) {
            values.insert(rule.input1, None);
        }
        if !values.contains_key(&rule.input2) {
            values.insert(rule.input2, None);
        }
        if !values.contains_key(&rule.output) {
            values.insert(rule.output, None);
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for rule in &rules {
            let input1 = values.get(&rule.input1).unwrap();
            let input2 = values.get(&rule.input2).unwrap();
            let output = values.get(&rule.output).unwrap();
            if input1.is_some() && input2.is_some() && output.is_none() {
                changed = true;
                let value1 = input1.unwrap();
                let value2 = input2.unwrap();
                let calculated = match rule.operator {
                    Operator::AND => value1 && value2,
                    Operator::OR => value1 || value2,
                    Operator::XOR => value1 ^ value2,
                };
                *values.get_mut(&rule.output).unwrap() = Some(calculated);
            }
        }
    }

    let mut total: u64 = 0;
    for value in &values {
        if value.0[0] == 'z' && value.1.unwrap() {
            let num_str = String::from_iter(value.0[1..].iter());
            let test = num_str.parse::<u32>().ok().unwrap();
            total |= 1 << test;
        }
    }

    println!("{total}");

    Ok(())
}
