use log::info;
use serde_json::Value;
use std::collections::VecDeque;

pub fn strip_non_json_characters(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut valid_json = String::new();
    let mut current_json = String::new();
    let mut stack = VecDeque::new();
    let mut in_string = false;
    let mut escape = false;

    for c in input.chars() {
        match c {
            '"' => {
                current_json.push(c);
                if !escape {
                    in_string = !in_string;
                }
                escape = false;
            }
            '\\' => {
                current_json.push(c);
                escape = !escape;
            }
            '{' | '[' => {
                if !in_string {
                    stack.push_back(c);
                }
                current_json.push(c);
            }
            '}' => {
                current_json.push(c);
                if !in_string {
                    if stack.back() == Some(&'{') {
                        stack.pop_back();
                        if stack.is_empty() {
                            valid_json.push_str(&current_json);
                            current_json.clear();
                        }
                    } else {
                        return Err("Mismatched closing brace".into());
                    }
                }
            }
            ']' => {
                current_json.push(c);
                if !in_string {
                    if stack.back() == Some(&'[') {
                        stack.pop_back();
                        if stack.is_empty() {
                            valid_json.push_str(&current_json);
                            current_json.clear();
                        }
                    } else {
                        return Err("Mismatched closing bracket".into());
                    }
                }
            }
            ':' | ',' => {
                current_json.push(c);
            }
            _ => {
                if !in_string && stack.is_empty() {
                    continue; // Skip characters outside JSON structures
                }
                current_json.push(c);
            }
        }
    }

    if in_string {
        return Err("Unclosed string".into());
    }

    if !stack.is_empty() {
        return Err("Unclosed structures".into());
    }
    Ok(valid_json)
}


pub fn format_to_json(input: &str) -> Result<String, serde_json::Error> {
    info!("Formatting to JSON.");

    let v: Value = serde_json::from_str(input)?;
    serde_json::to_string_pretty(&v)
}
