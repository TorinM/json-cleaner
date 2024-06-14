use std::io::{self, BufRead};

pub fn from_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Reading from file");
    Ok("Not implemented".to_string())
}

pub fn from_stdin() -> Result<String, Box<dyn std::error::Error>> {
    println!("Reading from stdin");

    let mut lines_vec: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        lines_vec.push(line);
    }

    Ok(lines_vec.concat())
}
