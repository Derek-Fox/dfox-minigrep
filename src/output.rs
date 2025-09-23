use std::path::Path;

use crate::search::{FileMatches, MatchedLine};

pub(crate) struct OutputFlags {
    color: bool,
    lines: bool,
    quiet: bool,
    count: bool,
}

impl OutputFlags {
    pub(crate) fn new(color: bool, lines: bool, quiet: bool, count: bool) -> Self {
        OutputFlags {
            color,
            lines,
            quiet,
            count,
        }
    }
}

pub(crate) fn output_matches(file_matches: Vec<FileMatches>, query: String, flags: &OutputFlags) {
    if flags.quiet {
        return;
    }

    let mut count = 0;

    for file in file_matches {
        let Some(mut file_name) = Path::new(&file.file_path).file_name().unwrap().to_str() else {
            //call 'unsafe' unwrap because how tf would we get to this point with /.. ???
            panic!("Invalid unicode in file_name");
        };

        if file_name == "-" {
            file_name = "stdin"
        }

        println!("-- {file_name} --");
        for line in file.matches {
            println!("{}", format_line(flags, line, query.len()));
            count += 1;
        }
    }

    if flags.count {
        println!("Number of matches: {count}");
    }
}

/**
 * Add formatting to a string. Specifically, prepend a line number and optionally color.
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
 * Merge overlapping or adjacent ranges. Each range is a (start, end) tuple.
 */
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

/**
 * Using ANSI escape sequence for red, colorize the range in line from [idx, idx+length).
 */
fn colorize_range(idx: usize, length: usize, line: &mut String) {
    let red = "\x1b[31m";
    let default = "\x1b[0m";

    line.insert_str(idx + length, default);
    line.insert_str(idx, red);
}
