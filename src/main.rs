use std::env;

mod read;
mod format;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: <file>");
        Err("Invalid arguments")?;
    }

    let input = if args.len() == 2 {
        read::from_file(&args[1])?
    } else {
        read::from_stdin()?
    };

    println!("Input: {}", input);

    Ok(())
}
