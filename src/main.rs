use log::{info, warn, LevelFilter};

mod read;
mod format;
mod cli;

fn set_log_level(verbose: bool) {
    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Off
    };

    env_logger::Builder::from_default_env()
        .filter_level(level)
        .init();
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = cli::set_args();

    let verbose = cli::get_flag(&args, "verbose");
    set_log_level(verbose);

    let input = match cli::get_arg(&args, "file")
    {
        Some(input) => read::from_file(&input)?,
        None => read::from_stdin()?
    };
    if input.is_empty() {
        warn!("No input provided.");
        return Err("No input provided.".into())
    }

    info!("Received input: `{}`", input);

    let cleaned_json_strings = match format::strip_non_json_characters(&input)
    {
        Ok(cleaned) => {
            info!("Cleaned input.");
            cleaned
        },
        Err(e) => {
            warn!("Error cleaning input: {}", e);
            return Err(e.into())
        }
    };
    if cleaned_json_strings.len() == 0 {
        warn!("No valid JSON found in input.");
        return Err("No valid JSON found in input.".into())
    }

    let formatted_json = format::format_valid_json(cleaned_json_strings)?;
    if formatted_json.is_empty() {
        warn!("No valid JSON found in input.");
        return Err("No valid JSON found in input.".into())
    }

    match cli::get_arg(&args, "output")
    {
        Some(output) => {
            info!("Writing to file {}.", output);
            std::fs::write(output, formatted_json)?
        },
        None => {
            info!("Writing to stdout.");
            println!("{}", formatted_json)
        }
    }
    Ok(())
}
