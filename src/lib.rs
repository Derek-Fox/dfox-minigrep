pub struct Match<'a> {
    line: &'a str,
    location: LocInfo,
}

struct LocInfo {
    line_number: usize,
    index: usize,
    query_len: usize,
}

/**
 * Search contents for instances of query. Returns a list of Match structs which capture the line
 * and information about the location of the match.
 */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<Match<'a>> {
    let mut matches: Vec<Match> = Vec::new();

    for (line_number, line) in contents.lines().enumerate() {
        let index = match line.find(query) {
            Some(idx) => idx,
            None => continue,
        };

        matches.push(Match {
            line,
            location: LocInfo {
                line_number,
                index,
                query_len: query.len(),
            },
        });
    }

    return matches;
}

/**
 * Add formatting to a string. Specifically, prepend a line number and optionally color.
 * Uses information supplied in Match struct, which represents the line and location of a match
 * found by fn search().
 */
pub fn format_line(color: bool, matched: Match) -> String {
    let line_num_str = format!("{} ", matched.location.line_number + 1);
    let mut formatted_line = format!("{line_num_str}{}", matched.line);

    if color {
        let adjusted_idx = line_num_str.len() + matched.location.index;
        colorize_range(
            adjusted_idx,
            matched.location.query_len,
            &mut formatted_line,
        );
    }

    return formatted_line;
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
