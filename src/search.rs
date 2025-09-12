pub(crate) struct MatchedLine<'a> {
    pub(crate) line: &'a str,
    pub(crate) line_number: u32,
    pub(crate) locations: Vec<usize>,
}

impl<'a> MatchedLine<'a> {
    pub(crate) fn new(line: &'a str, line_number: u32) -> Self {
        MatchedLine {
            line,
            line_number,
            locations: Vec::new(),
        }
    }
}

pub(crate) struct SearchFlags {
    case_insensitive: bool,
}

impl SearchFlags {
    pub(crate) fn new(case_insensitive: bool) -> Self {
        SearchFlags { case_insensitive }
    }
}

/**
 * Search contents for instances of query. Returns a list of Match structs which capture the line
 * and information about the location of the match.
 */
pub(crate) fn search<'a>(
    query: &str,
    contents: &'a str,
    flags: &SearchFlags,
) -> Vec<MatchedLine<'a>> {
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
