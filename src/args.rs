use crate::{
    output::{OutputFlags, output_matches},
    search::{SearchFlags, search_dir},
};
use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;

pub fn run(config: Config) {
    if let Some(matches) = search_dir(&config.query, &config.file_path, &config.search_flags) {
        output_matches(matches, config.query, &config.output_flags);
    }
}

pub fn parse_args() -> ArgMatches {
    Command::new("minigrep")
        .arg(Arg::new("query").required(true))
        .arg(Arg::new("file").required(true))
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
    file_path: PathBuf,
    search_flags: SearchFlags,
    output_flags: OutputFlags,
}

impl Config {
    pub fn new(args: &clap::ArgMatches) -> Self {
        Config {
            query: args.get_one::<String>("query").unwrap().to_string(),
            file_path: PathBuf::from(args.get_one::<String>("file").map_or("-", |v| v)),

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
