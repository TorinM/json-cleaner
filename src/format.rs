use regex::Regex;
use serde_json::Value;

pub fn strip_non_json_characters(input: &str) -> String {
    let re = Regex::new(r"[^\x20-\x7E]").unwrap();
    re.replace_all(input, "").to_string()
}

pub fn format_to_json(input: &str) -> Result<String, serde_json::Error> {
    let v: Value = serde_json::from_str(input)?;
    serde_json::to_string_pretty(&v)
}