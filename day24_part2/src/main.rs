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
    let (mut values, mut rules) = parse_file("data.txt")?;

    let bits = values.len() / 2;

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

    let mut outputs: Vec<Option<Rule>> = vec![None; bits];
    for rule in &rules {
        if rule.output[0] != 'z' {
            continue;
        }
        if rule.operator != Operator::XOR {
            continue;
        }
        let output_str = String::from_iter(rule.output[1..].iter());
        let value = output_str.parse::<usize>().ok();
        if value.is_none() {
            continue;
        }
        let index = value.unwrap();
        outputs[index] = Some(rule.clone());
    }

    let mut input_ands: Vec<Option<Rule>> = vec![None; bits];
    let mut input_xors: Vec<Option<Rule>> = vec![None; bits];
    for rule in &rules {
        let num_str1 = String::from_iter(rule.input1[1..].iter());
        let value1 = num_str1.parse::<usize>().ok();
        if value1.is_none() {
            continue;
        }
        let num_str2 = String::from_iter(rule.input2[1..].iter());
        let value2 = num_str2.parse::<usize>().ok();
        if value2.is_none() {
            continue;
        }
        let index1: usize = value1.unwrap();
        let index2: usize = value2.unwrap();
        if index1 != index2 {
            continue;
        }
        if rule.input1[0] != 'x' && rule.input1[0] != 'y' {
            continue;
        }
        if rule.input2[0] != 'x' && rule.input2[0] != 'y' {
            continue;
        }
        if rule.input1[0] == rule.input2[0] {
            continue;
        }

        if rule.operator == Operator::AND {
            input_ands[index1] = Some(rule.clone());
        } else if rule.operator == Operator::XOR {
            input_xors[index2] = Some(rule.clone());
        }
    }
    let mut swaps: Vec<[char; 3]> = vec![];
    let total_rules = rules.len();
    let mut problems: Vec<bool> = vec![false; bits + 1];
    for i in 0..bits {
        if outputs[i].is_none() {
            let output = input_xors[i].clone().unwrap().output;
            let mut index1: usize = 0;
            let mut index2: usize = 0;

            for rule_index in 0..total_rules {
                let rule: &Rule = &rules[rule_index];
                if rule.operator != Operator::XOR {
                    continue;
                }
                if output == rule.input1 || output == rule.input2 {
                    index1 = rule_index;
                    break;
                }
            }
            for rule_index in 0..total_rules {
                let rule2 = &rules[rule_index];
                if rule2.output[0] != 'z' {
                    continue;
                }
                if rule2.operator != Operator::XOR {
                    continue;
                }
                let output_str = String::from_iter(rule2.output[1..].iter());
                let value = output_str.parse::<usize>().ok();
                if value.is_none() {
                    continue;
                }
                let index = value.unwrap();
                if index == i {
                    index2 = rule_index;
                    break;
                }
            }
            swaps.push(rules[index1].output.clone());
            swaps.push(rules[index2].output.clone());

            let temp = rules[index2].output;
            rules[index2].output = rules[index1].output;
            rules[index1].output = temp;
            problems[i] = true;
            outputs[i] = Some(rules[index2].clone());
        }
    }

    let mut carries: Vec<Option<[char; 3]>> = vec![None; bits + 1];
    for i in 1..bits {
        let check_output = input_xors[i].clone().unwrap().output;
        if outputs[i].clone().unwrap().input1 == check_output {
            carries[i - 1] = Some(outputs[i].clone().unwrap().input2);
        } else if outputs[i].clone().unwrap().input2 == check_output {
            carries[i - 1] = Some(outputs[i].clone().unwrap().input1);
        } else if !problems[i] {
            swaps.push(check_output);
            swaps.push(outputs[i].clone().unwrap().input2);
        }
    }
    swaps.sort();

    for i in 0..8 {
        print!("{}{}{},", swaps[i][0], swaps[i][1], swaps[i][2]);
    }
    println!();
    return Ok(());
}
