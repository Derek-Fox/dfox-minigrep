use crate::{
    output::{OutputFlags, output_matches},
    search::{SearchFlags, search_dir},
};
use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;

pub fn run(config: Config) {
    if let Some(matches) = search_dir(&config.query, &config.path, &config.search_flags) {
        output_matches(matches, config.query, config.output_flags);
    }
}

pub fn parse_args() -> ArgMatches {
    Command::new("minigrep")
        .arg(Arg::new("query").required(true))
        .arg(Arg::new("path").required(true))
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
        .arg(
            Arg::new("num_threads")
                .long("num_threads")
                .required(false)
                .help("Specify the number of threads the program will utilize"),
        )
        .get_matches()
}

pub struct Config {
    query: String,
    path: PathBuf,
    search_flags: SearchFlags,
    output_flags: OutputFlags,
}

impl Config {
    pub fn new(args: &clap::ArgMatches) -> Self {
        Config {
            query: args.get_one::<String>("query").unwrap().to_string(),
            path: PathBuf::from(args.get_one::<String>("path").map_or("-", |v| v)),

            output_flags: OutputFlags {
                color: !args.get_flag("no-color"),
                lines: !args.get_flag("no-lines"),
                quiet: args.get_flag("quiet"),
                count: args.get_flag("count"),
            },
            search_flags: SearchFlags {
                case_insensitive: args.get_flag("case-insensitive"),
            },
        }
    }
}
