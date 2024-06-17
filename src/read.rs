use log::{info, warn};
use std::io::{self, BufRead};
use std::fs::File;

fn read_lines<R: BufRead>(reader: R) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = String::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                warn!("Error reading line: {}", e);
                return Err(Box::new(e));
            }
        };
        result.push_str(&line);
    }
    Ok(result)
}

pub fn from_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    info!("Reading input from file {}.", file_path);

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let content = read_lines(reader)?;

    info!("Finished reading input from file.");

    Ok(content)
}


pub fn from_stdin() -> Result<String, Box<dyn std::error::Error>> {
    info!("Reading input from standard stdin.");

    let stdin = io::stdin().lock();
    let result = read_lines(stdin)?;

    info!("Finished reading input from stdin.");

    Ok(result)
}
