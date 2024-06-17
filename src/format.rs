use log::info;
use serde_json::Value;
use std::collections::VecDeque;

pub fn strip_non_json_characters(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut valid_json = String::new();
    let mut stack = VecDeque::new();
    let mut in_string = false;
    let mut escape = false;

    for c in input.chars() {
        match c {
            '"' => {
                if !escape {
                    in_string = !in_string;
                }
                escape = false;
                valid_json.push(c);
            }
            '\\' => {
                escape = !escape;
                valid_json.push(c);
            }
            '{' | '[' => {
                if !in_string {
                    stack.push_back(c);
                }
                valid_json.push(c);
            }
            '}' => {
                if !in_string {
                    if stack.back() == Some(&'{') {
                        stack.pop_back();
                    } else {
                        return Err("Mismatched closing brace".into());
                    }
                }
                valid_json.push(c);
            }
            ']' => {
                if !in_string {
                    if stack.back() == Some(&'[') {
                        stack.pop_back();
                    } else {
                        return Err("Mismatched closing bracket".into());
                    }
                }
                valid_json.push(c);
            }
            ':' => {
                if !in_string && (valid_json.ends_with('"') || valid_json.ends_with('}') || valid_json.ends_with(']')) {
                    valid_json.push(c);
                } else {
                    return Err("Invalid key-value separator".into());
                }
            }
            ',' => {
                if !in_string && (valid_json.ends_with('}') || valid_json.ends_with(']') || valid_json.chars().last().map_or(false, |c| c.is_ascii_digit())) {
                    valid_json.push(c);
                } else {
                    return Err("Invalid value separator".into());
                }
            }
            _ => {
                if !in_string && c.is_whitespace() {
                    continue;
                }
                valid_json.push(c);
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
