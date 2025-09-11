use clap::{Arg, ArgMatches, Command};
use minigrep::FormatFlags;
use minigrep::format_line;
use minigrep::search;
use std::error::Error;
use std::fs;
use std::io::{self, Read};
use std::process;
use std::time::Instant;

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
            Arg::new("bench")
                .long("bench")
                .action(clap::ArgAction::SetTrue)
                .help("Run a basic performance benchmark"),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .action(clap::ArgAction::SetTrue)
                .help("Suppress output"),
        )
        .get_matches();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if args.get_flag("bench") {
        run_bench(&config);
        return;
    }

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

    let matched_lines = search(&config.query, &contents);

    if config.quiet {
        return Ok(());
    }

    for matched_line in matched_lines {
        println!(
            "{}",
            format_line(&config.format_flags, matched_line, config.query.len())
        );
    }

    Ok(())
}

fn run_bench(config: &Config) {
    let iterations = 10;
    let contents = if config.file_path == "-" {
        let mut buff = String::new();
        io::stdin().read_to_string(&mut buff).unwrap();
        buff
    } else {
        fs::read_to_string(&config.file_path).unwrap()
    };

    let start = Instant::now();
    let mut total_matches = 0;
    for _ in 0..iterations {
        let matched_lines = search(&config.query, &contents);
        total_matches += matched_lines.len();
    }
    let duration = start.elapsed();
    println!(
        "Searched '{}' in '{}' {} times in {:.2?} (avg: {:.2?} per run, total matches: {})",
        config.query,
        config.file_path,
        iterations,
        duration,
        duration / iterations,
        total_matches
    );
}

struct Config {
    query: String,
    file_path: String,
    quiet: bool,
    format_flags: FormatFlags,
}

impl Config {
    pub fn build(args: &ArgMatches) -> Result<Self, &'static str> {
        Ok(Config {
            query: args.get_one::<String>("query").unwrap().clone(),
            file_path: args
                .get_one::<String>("file")
                .map(|s| s.as_str())
                .unwrap_or("-")
                .to_string(),
            quiet: args.get_flag("quiet"),
            format_flags: FormatFlags::new(!args.get_flag("no-color"), !args.get_flag("no-lines")),
        })
    }
}
