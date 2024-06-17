use log::{info, warn};
use serde_json::Value;
use std::collections::VecDeque;

pub fn strip_non_json_characters(input: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut valid_json_strings: Vec<String> = Vec::new();
    let mut current_json = String::new();
    let mut stack = VecDeque::new();
    let mut in_string = false;
    let mut escape = false;
    let mut start_primitive = false;

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
                    start_primitive = false;
                }
                current_json.push(c);
            }
            '}' => {
                current_json.push(c);
                if !in_string {
                    if stack.back() == Some(&'{') {
                        stack.pop_back();
                        if stack.is_empty() {
                            valid_json_strings.push(current_json.clone());
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
                            valid_json_strings.push(current_json.clone());
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
                    if c.is_whitespace() {
                        continue; // Skip characters outside JSON structures
                    }
                    if start_primitive {
                        current_json.push(c);
                    } else if c == '-' || c.is_digit(10) || c == 't' || c == 'f' || c == 'n' {
                        start_primitive = true;
                        current_json.push(c);
                    }
                } else {
                    current_json.push(c);
                }
            }
        }
    }

    if in_string {
        return Err("Unclosed string".into());
    }

    if !stack.is_empty() {
        return Err("Unclosed structures".into());
    }

    // Check if we have a valid primitive value
    if start_primitive && !current_json.is_empty() {
        valid_json_strings.push(current_json.clone());
    }

    Ok(valid_json_strings)
}


pub fn format_valid_json(input: Vec<String>) -> Result<String, serde_json::Error> {
    info!("Formatting valid JSON.");

    let mut formatted_json = String::new();
    for json_str in input {
        match serde_json::from_str::<Value>(&json_str) {
            Ok(value) => {
                let formatted = serde_json::to_string_pretty(&value)?;
                formatted_json.push_str(&formatted);
            }
            Err(_) => {
                warn!("Skipping invalid JSON: {}", json_str);
            }
        }
    }
    Ok(formatted_json)
}
