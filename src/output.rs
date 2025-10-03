use crate::search::{FileMatches, MatchedLine};
use colored::*; // Add this import

pub(crate) struct OutputFlags {
    pub(crate) color: bool,
    pub(crate) lines: bool,
    pub(crate) quiet: bool,
    pub(crate) count: bool,
}

pub(crate) fn output_matches(file_matches: Vec<FileMatches>, query: String, flags: OutputFlags) {
    if flags.quiet {
        std::process::exit(if file_matches.is_empty() { 1 } else { 0 });
    }

    let mut count = 0;

    for file in file_matches {
        println!("-- {} --", file.file_path.display());
        for line in file.matches {
            println!("{}", format_line(&flags, line, &query));
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
fn format_line(flags: &OutputFlags, matched: MatchedLine, query: &str) -> String {
    let mut line = String::from(matched.line);

    if flags.color {
        let ranges: Vec<(usize, usize)> = matched
            .locations
            .iter()
            .map(|&loc| (loc, loc + query.len()))
            .collect();
        let merged = merge_ranges(ranges);
        line = colorize_ranges(&line, &merged);
    }

    if flags.lines {
        line = format!("{:04}] {}", matched.line_number, line);
    }

    line
}

/**
 * Merge overlapping or adjacent ranges. Each range is a (start, end) tuple.
 */
fn merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if ranges.is_empty() {
        return vec![];
    }
    ranges.sort_by_key(|r| r.0);
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
    merged
}

/**
 * Colorize all ranges in the line using the colored crate.
 */
fn colorize_ranges(line: &str, ranges: &[(usize, usize)]) -> String {
    if ranges.is_empty() {
        return line.to_string();
    }

    let mut result = String::new();
    let mut last = 0;
    for &(start, end) in ranges {
        // Ensure valid char boundaries
        let start = line
            .char_indices()
            .nth(start)
            .map(|(i, _)| i)
            .unwrap_or(line.len());
        let end = line
            .char_indices()
            .nth(end)
            .map(|(i, _)| i)
            .unwrap_or(line.len());

        if last < start {
            result.push_str(&line[last..start]);
        }
        result.push_str(&line[start..end].red().to_string());
        last = end;
    }
    if last < line.len() {
        result.push_str(&line[last..]);
    }
    result
}
