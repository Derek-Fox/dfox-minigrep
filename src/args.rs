use crate::{
    output::{OutputFlags, output_matches},
    search::{FileMatches, SearchFlags, search_contents, search_dir},
};
use clap::{Arg, ArgMatches, Command};
use std::io::{self, Read};

fn handle_stdin(config: &Config) -> Option<Vec<FileMatches>> {
    let mut buff = String::new();
    io::stdin()
        .read_to_string(&mut buff)
        .expect("Invalid UTF-8");
    if let Some(matches) =
        search_contents(&config.query, buff, &config.file_path, &config.search_flags)
    {
        return Some(vec![matches]);
    }
    None
}

pub fn run(config: Config) {
    let results = if config.file_path == "-" {
        handle_stdin(&config)
    } else {
        search_dir(&config.query, config.file_path, &config.search_flags)
    };

    if let Some(matches) = results {
        output_matches(matches, config.query, &config.output_flags);
    }
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
            query: args.get_one::<String>("query").unwrap().to_string(),
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
