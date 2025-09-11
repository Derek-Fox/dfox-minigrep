use clap::{Arg, ArgMatches, Command};
use std::{
    error::Error,
    fs,
    io::{self, Read},
};

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
            ),
            search_flags: SearchFlags::new(args.get_flag("case-insensitive")),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = if config.file_path == "-" {
        let mut buff = String::new();
        io::stdin().read_to_string(&mut buff)?;
        buff
    } else {
        fs::read_to_string(config.file_path)?
    };

    let query_len = config.query.len();

    let matched_lines = search(&config.query, &contents, &config.search_flags);

    if config.output_flags.quiet {
        return Ok(());
    }

    for matched_line in matched_lines {
        println!(
            "{}",
            format_line(&config.output_flags, matched_line, query_len)
        );
    }

    Ok(())
}

struct MatchedLine<'a> {
    line: &'a str,
    line_number: u32,
    locations: Vec<usize>,
}

impl<'a> MatchedLine<'a> {
    /// Create a MatchedLine with an empty locations vec
    pub fn new(line: &'a str, line_number: u32) -> Self {
        MatchedLine {
            line,
            line_number,
            locations: Vec::new(),
        }
    }
}

pub struct SearchFlags {
    case_insensitive: bool,
}

impl SearchFlags {
    pub fn new(case_insensitive: bool) -> Self {
        SearchFlags { case_insensitive }
    }
}

/**
 * Search contents for instances of query. Returns a list of Match structs which capture the line
 * and information about the location of the match.
 */
fn search<'a>(query: &str, contents: &'a str, flags: &SearchFlags) -> Vec<MatchedLine<'a>> {
    if query.is_empty() {
        return Vec::new();
    }

    let mut matched_lines: Vec<MatchedLine> = Vec::new();
    let mut query = query.to_string();
    if flags.case_insensitive {
        query.make_ascii_lowercase();
    }

    for (line_number, line) in contents.lines().enumerate() {
        let mut matched_line = MatchedLine::new(line, line_number as u32);

        let mut line = line.to_string();
        if flags.case_insensitive {
            line.make_ascii_lowercase();
        }

        let mut start = 0;
        while let Some(index) = line[start..].find(&query) {
            let idx = start + index;
            matched_line.locations.push(idx);
            start = idx + 1;
        }

        if !matched_line.locations.is_empty() {
            matched_lines.push(matched_line);
        }
    }
    return matched_lines;
}

/// Merge overlapping or adjacent ranges.
/// Each range is a (start, end) tuple.
fn merge_ranges(ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut merged = Vec::new();
    let mut current = ranges[0];

    for range in ranges.into_iter().skip(1) {
        if range.0 <= current.1 {
            current.1 = current.1.max(range.1);
        } else {
            merged.push(current);
            current = range;
        }
    }
    merged.push(current);
    return merged;
}

struct OutputFlags {
    color: bool,
    lines: bool,
    quiet: bool,
}

impl OutputFlags {
    pub fn new(color: bool, lines: bool, quiet: bool) -> Self {
        OutputFlags {
            color,
            lines,
            quiet,
        }
    }
}

/**
 * Add formatting to a string. Specifically, prepend a line number and optionally color.
 * Uses information supplied in Match struct, which represents the line and location of a match
 * found by fn search().
 */
fn format_line(flags: &OutputFlags, matched: MatchedLine, query_len: usize) -> String {
    let mut line = String::from(matched.line);

    if flags.color {
        let ranges: Vec<(usize, usize)> = matched
            .locations
            .iter()
            .map(|&loc| (loc, loc + query_len))
            .collect();
        let merged = merge_ranges(ranges);
        for (start, end) in merged.into_iter().rev() {
            colorize_range(start, end - start, &mut line);
        }
    }

    if flags.lines {
        line = format!("{:04}] {}", matched.line_number + 1, line);
    }
    return line;
}

/**
 * Using ANSI escape sequence for red, colorize the range in line from [idx, idx+length).
 */
fn colorize_range(idx: usize, length: usize, line: &mut String) {
    let red = "\x1b[31m";
    let default = "\x1b[0m";

    line.insert_str(idx + length, default);
    line.insert_str(idx, red);
}
