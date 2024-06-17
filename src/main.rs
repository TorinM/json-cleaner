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
    info!("Received input: {}", input);

    let cleaned = match format::strip_non_json_characters(&input)
    {
        Ok(cleaned) => {
            info!("Cleaned input: {}", cleaned);
            cleaned
        },
        Err(e) => {
            warn!("Error cleaning input: {}", e);
            return Err(e.into())
        }
    };

    let json = format::format_to_json(&cleaned)?;

    match cli::get_arg(&args, "output")
    {
        Some(output) => {
            info!("Writing to file {}.", output);
            std::fs::write(output, json)?
        },
        None => {
            info!("Writing to stdout.");
            println!("{}", json)
        }
    }
    Ok(())
}
