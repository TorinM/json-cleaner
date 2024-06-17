use clap::{Arg, Command, ArgAction};

pub fn set_args() -> clap::ArgMatches {
    Command::new("json-cleaner")
    .version("1.0")
    .author("Torin May <torinmay@gmail.com>")
    .about("A command line tool for easily cleaning input into valid JSON.")
    .arg(
        Arg::new("file")
            .help("Sets the file to read from. Optional. Default: stdin.")
            .value_name("FILE")
            .index(1)
    ).arg(
        Arg::new("output")
            .short('o')
            .long("out")
            .value_name("OUTPUT")
            .help("Sets the name of the output file. Optional. Default: stdout.")
    ).arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Sets the level of verbosity. Optional. Default: false.")
            .action(ArgAction::SetTrue)
    )
    .get_matches()
}

pub fn get_arg(matches: &clap::ArgMatches, name: &str) -> Option<String> {
    matches.get_one::<String>(name).cloned()
}

pub fn get_flag(matches: &clap::ArgMatches, name: &str) -> bool {
    matches.get_flag(name)
}
