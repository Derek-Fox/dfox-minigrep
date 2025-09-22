use std::fs;

pub(crate) struct FileMatches {
    pub(crate) file_path: String,
    pub(crate) matches: Vec<MatchedLine>,
}
pub(crate) struct MatchedLine {
    pub(crate) line: String,
    pub(crate) line_number: u32,
    pub(crate) locations: Vec<usize>,
}

impl MatchedLine {
    pub(crate) fn new(line: String, line_number: u32) -> Self {
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
 * Recursively search directory. Searches all files in the directory, and searches any/all subdirectories.
 */
pub(crate) fn search_dir(
    query: &String,
    file_path: String,
    flags: &SearchFlags,
) -> Vec<FileMatches> {
    if fs::metadata(&file_path)
        .unwrap_or_else(|e| panic!("Error: {} with path {}", e, file_path))
        .is_file()
    {
        let contents = fs::read_to_string(&file_path).unwrap();
        let matches = search_contents(query, contents, &file_path, flags);
        return vec![matches];
    }

    let dir_iter = fs::read_dir(file_path).unwrap();
    let mut dir_results: Vec<_> = Vec::new();
    for dir in dir_iter {
        let canon_path = dir
            .unwrap()
            .path()
            .canonicalize()
            .unwrap_or_else(|e| panic!("Error: {}", e));
        let path_str = canon_path
            .to_str()
            .unwrap_or_else(|| panic!("Path with invalid unicode."));
        dir_results.append(&mut search_dir(query, path_str.to_string(), &flags));
    }
    return dir_results;
}

/**
 * Search contents for instances of query. Returns a list of MatchedLine structs which capture the line
 * and information about the location of the match.
 */
pub(crate) fn search_contents(
    query: &String,
    contents: String,
    file_path: &String,
    flags: &SearchFlags,
) -> FileMatches {
    if query.is_empty() {
        return FileMatches {
            file_path: file_path.to_string(),
            matches: Vec::new(),
        };
    }

    let mut matched_lines: Vec<MatchedLine> = Vec::new();
    let mut query = query.to_string();
    if flags.case_insensitive {
        query.make_ascii_lowercase();
    }

    for (line_number, line) in contents.lines().enumerate() {
        let mut matched_line = MatchedLine::new(line.to_string(), line_number as u32);

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
    return FileMatches {
        matches: matched_lines,
        file_path: file_path.to_string(),
    };
}
