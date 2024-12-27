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

fn build_order(
    values: &mut HashMap<[char; 3], Option<bool>>,
    rules: &Vec<Rule>,
    number_bits: usize,
) -> Vec<Rule> {
    clear_values(values, number_bits);
    let mut order: Vec<Rule> = Vec::new();
    let mut changed = true;
    while changed {
        changed = false;
        for rule in rules {
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
                order.push(rule.clone());
            }
        }
    }

    order
}

fn calculate_value(
    values: &mut HashMap<[char; 3], Option<bool>>,
    rules: &Vec<Rule>,
    bits: usize,
    swaps: &HashMap<[char; 3], [char; 3]>,
) -> Vec<Option<usize>> {
    let mut changed = true;
    while changed {
        changed = false;
        for rule in rules {
            let input1 = values.get(&rule.input1).unwrap();
            let input2 = values.get(&rule.input2).unwrap();
            let output = values.get(&rule.output).unwrap();
            if input1.is_some() && input2.is_some() {
                let value1 = input1.unwrap();
                let value2 = input2.unwrap();
                let calculated = match rule.operator {
                    Operator::AND => value1 && value2,
                    Operator::OR => value1 || value2,
                    Operator::XOR => value1 ^ value2,
                };

                let new_output = swaps.get(&rule.output);

                if new_output.is_none() {
                    if output.is_none() {
                        changed = true;
                        *values.get_mut(&rule.output).unwrap() = Some(calculated);
                    }
                } else {
                    let other_output = values.get(new_output.unwrap()).unwrap();
                    if other_output.is_none() {
                        changed = true;
                        *values.get_mut(new_output.unwrap()).unwrap() = Some(calculated);
                    }
                }
            }
        }
    }

    let mut bit_values: Vec<Option<usize>> = vec![None; bits];
    for value in values {
        if value.0[0] == 'z' {
            let num_str = String::from_iter(value.0[1..].iter());
            let test = num_str.parse::<usize>().ok().unwrap();
            bit_values[test] = if value.1.is_none() {
                None
            } else {
                Some(if value.1.unwrap() { 1 } else { 0 })
            };
        }
    }
    bit_values
}

fn clear_values(values: &mut HashMap<[char; 3], Option<bool>>, bits: usize) {
    for value in &mut *values {
        *value.1 = None;
    }
    for i in 0..bits {
        let x: [char; 3] = [
            'x',
            char::from_digit((i / 10) as u32, 10).unwrap(),
            char::from_digit((i % 10) as u32, 10).unwrap(),
        ];
        if let Some(val) = values.get_mut(&x) {
            *val = Some(false);
        }
        let y: [char; 3] = [
            'y',
            char::from_digit((i / 10) as u32, 10).unwrap(),
            char::from_digit((i % 10) as u32, 10).unwrap(),
        ];
        if let Some(val) = values.get_mut(&y) {
            *val = Some(false);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut values, rules) = parse_file("data.txt")?;

    let number_bits: usize = values.len() / 2;
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

    let mut outputs: Vec<[char; 3]> = vec![];
    for rule in &rules {
        outputs.push(rule.output.clone());
    }
    let number_outputs = outputs.len();

    let mut swap_combinations: Vec<HashMap<[char; 3], [char; 3]>> = vec![];
    for i in 0..number_outputs {
        for j in (i + 1)..number_outputs {
            let mut hash_map: HashMap<[char; 3], [char; 3]> = HashMap::new();
            hash_map.insert(outputs[i], outputs[j]);
            hash_map.insert(outputs[j], outputs[i]);
            swap_combinations.push(hash_map);
        }
    }

    let order = build_order(&mut values, &rules, number_bits);

    for i in 0..(number_bits - 2) {
        println!("{}", i);
        let hash_map: HashMap<[char; 3], [char; 3]> = HashMap::new();
        let base_differences = all_differences(&hash_map, number_bits, &mut values, &order, i);

        if base_differences != 0 {
            for swaps in &swap_combinations {
                let differences = all_differences(swaps, number_bits, &mut values, &order, i);

                if differences == 0 {
                    println!("{:?}", swaps);
                }
            }
        }
    }

    println!("cgq,fnr,kqk,nbc,svm,z15,z23,z39"); // Manual looking through the test cases above

    Ok(())
}

fn all_differences(
    swaps: &HashMap<[char; 3], [char; 3]>,
    number_bits: usize,
    values: &mut HashMap<[char; 3], Option<bool>>,
    order: &Vec<Rule>,
    j: usize,
) -> usize {
    let mut total = 0;

    for x0 in 0..2 {
        for x1 in 0..2 {
            for y0 in 0..2 {
                for y1 in 0..2 {
                    clear_values(values, number_bits);
                    set_x(values, j, x0);
                    set_x(values, j + 1, x1);
                    set_y(values, j, y0);
                    set_y(values, j + 1, y1);
                    let bits = calculate_value(values, &order, number_bits + 1, swaps);

                    let sub = x0 + y0 + x1 * 2 + y1 * 2;
                    let mut compare_bits: Vec<usize> = vec![0; number_bits + 1];
                    compare_bits[j] = sub & 1;
                    compare_bits[j + 1] = (sub & 2) >> 1;
                    compare_bits[j + 2] = (sub & 4) >> 2;

                    let differences = compare_sets(&bits, &compare_bits);
                    total += differences;
                }
            }
        }
    }
    total
}

fn compare_sets(calculated: &Vec<Option<usize>>, correct: &Vec<usize>) -> usize {
    let total: usize = calculated
        .iter()
        .zip(correct)
        .map(|item| {
            if item.0.is_none() {
                1
            } else {
                let value = item.0.unwrap();
                if value == *item.1 {
                    0
                } else {
                    1
                }
            }
        })
        .sum();
    total
}

fn set_x(values: &mut HashMap<[char; 3], Option<bool>>, bit: usize, value: usize) {
    let x: [char; 3] = [
        'x',
        char::from_digit((bit / 10) as u32, 10).unwrap(),
        char::from_digit((bit % 10) as u32, 10).unwrap(),
    ];
    if let Some(val) = values.get_mut(&x) {
        *val = Some(if value == 0 { false } else { true });
    }
}

fn set_y(values: &mut HashMap<[char; 3], Option<bool>>, bit: usize, value: usize) {
    let x: [char; 3] = [
        'y',
        char::from_digit((bit / 10) as u32, 10).unwrap(),
        char::from_digit((bit % 10) as u32, 10).unwrap(),
    ];
    if let Some(val) = values.get_mut(&x) {
        *val = Some(if value == 0 { false } else { true });
    }
}
