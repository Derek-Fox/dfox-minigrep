use clap::{Arg, ArgMatches, Command};
use minigrep::{OutputFlags, SearchFlags};
use std::{
    error::Error,
    fs,
    io::{self, Read},
    process,
};

fn main() {
    let args = Command::new("minigrep")
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
        .get_matches();

    let config = Config::new(&args);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = if config.file_path == "-" {
        let mut buff = String::new();
        io::stdin().read_to_string(&mut buff)?;
        buff
    } else {
        fs::read_to_string(config.file_path)?
    };

    let query_len = config.query.len();

    let matched_lines = minigrep::search(&config.query, &contents, &config.search_flags);

    if config.output_flags.quiet {
        return Ok(());
    }

    for matched_line in matched_lines {
        println!(
            "{}",
            minigrep::format_line(&config.output_flags, matched_line, query_len)
        );
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    search_flags: SearchFlags,
    output_flags: OutputFlags,
}

impl Config {
    pub fn new(args: &ArgMatches) -> Self {
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
            ),
            search_flags: SearchFlags::new(args.get_flag("case-insensitive")),
        }
    }
}
