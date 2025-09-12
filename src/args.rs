use crate::{
    output::{self, OutputFlags},
    search::{self, SearchFlags},
};
use clap::{Arg, ArgMatches, Command};
use std::{
    error::Error,
    fs,
    io::{self, Read},
};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = if config.file_path == "-" {
        let mut buff = String::new();
        io::stdin().read_to_string(&mut buff)?;
        buff
    } else {
        fs::read_to_string(config.file_path)?
    };

    let matched_lines = search::search(&config.query, &contents, &config.search_flags);
    output::output(matched_lines, config.query, &config.output_flags);

    Ok(())
}

pub fn parse_args() -> ArgMatches {
    Command::new("minigrep")
        .arg(Arg::new("query").required(true))
        .arg(Arg::new("file").required(false))
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .action(clap::ArgAction::SetTrue)
                .help("Disable colored output"),
        )
        .arg(
            Arg::new("no-lines")
                .long("no-lines")
                .action(clap::ArgAction::SetTrue)
                .help("Disable line numbers"),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .action(clap::ArgAction::SetTrue)
                .help("Suppress output"),
        )
        .arg(
            Arg::new("case-insensitive")
                .long("case-insensitive")
                .short('i')
                .action(clap::ArgAction::SetTrue)
                .help("Case insensitive searching"),
        )
        .arg(
            Arg::new("count")
                .long("count")
                .short('c')
                .action(clap::ArgAction::SetTrue)
                .help("Output a count of matches found"),
        )
        .get_matches()
}

pub struct Config {
    query: String,
    file_path: String,
    search_flags: SearchFlags,
    output_flags: OutputFlags,
}

impl Config {
    pub fn new(args: &clap::ArgMatches) -> Self {
        Config {
            query: args.get_one::<String>("query").unwrap().clone(),
            file_path: args
                .get_one::<String>("file")
                .map(|s| s.as_str())
                .unwrap_or("-")
                .to_string(),

            output_flags: OutputFlags::new(
                !args.get_flag("no-color"),
                !args.get_flag("no-lines"),
                args.get_flag("quiet"),
                args.get_flag("count"),
            ),
            search_flags: SearchFlags::new(args.get_flag("case-insensitive")),
        }
    }
}
