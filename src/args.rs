use crate::{
    output::{output_matches, OutputFlags},
    search::{search_dir, PreparedQuery, SearchFlags},
};
use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::{borrow::Cow, path::PathBuf};

pub(crate) fn prepare_query<'a>(query: &'a str, flags: &SearchFlags) -> Option<PreparedQuery<'a>> {
    if flags.regexp {
        Regex::new(query).ok().map(PreparedQuery::Regex)
    } else if flags.case_insensitive {
        Some(PreparedQuery::Plain(Cow::Owned(query.to_lowercase())))
    } else {
        Some(PreparedQuery::Plain(Cow::Borrowed(query)))
    }
}

pub fn run(config: Config) {
    let prepared_query = prepare_query(&config.query, &config.search_flags)
        .expect("Invalid regex encountered.");
    if let Some(matches) = search_dir(&prepared_query, &config.path, &config.search_flags) {
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
            Arg::new("regexp")
                .long("regexp")
                .short('p')
                .action(clap::ArgAction::SetTrue)
                .help("Match against regular expression"),
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
                regexp: args.get_flag("regexp"),
            },
        }
    }
}
