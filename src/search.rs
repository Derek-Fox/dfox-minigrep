use std::fs::{self, DirEntry};

pub(crate) struct FileMatches {
    pub(crate) file_path: String,
    pub(crate) matches: Vec<MatchedLine>,
}

pub(crate) struct MatchedLine {
    pub(crate) line: String,
    pub(crate) line_number: u32,
    pub(crate) locations: Vec<usize>,
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
 * Check metadata of file_path to see if it is a file. Returns false on failure to access metadata.
 */
fn is_file(file_path: &String) -> bool {
    match fs::metadata(&file_path) {
        Err(e) => {
            eprintln!("Error occured when trying to access metadata of {file_path}: {e}");
            return false;
        }
        Ok(metadata) => metadata.is_file(),
    }
}

/**
 * Get the canonical file path as a string.
 */
fn canonical_path_str(dir_entry: DirEntry) -> Option<String> {
    let Ok(canon_path) = dir_entry.path().canonicalize() else {
        return None;
    };
    Some(canon_path.to_string_lossy().to_string())
}

/**
 * Recursively search directory. Searches all files in the directory, and searches any/all subdirectories.
 */
pub(crate) fn search_dir(
    query: &String,
    file_path: String,
    flags: &SearchFlags,
) -> Option<Vec<FileMatches>> {
    /* First check if we have a file. If so, try to read its contents and get matches. Base case of recursion. */
    if is_file(&file_path) {
        let Ok(contents) = fs::read_to_string(&file_path) else {
            return None; // file doesn't contain valid UTF-8 - just ignore it
        };
        if let Some(matches) = search_contents(query, contents, &file_path, flags) {
            return Some(vec![matches]);
        } else {
            return None;
        }
    }

    /* Get an iterator over contents of directory */
    let dir_iter = match fs::read_dir(&file_path) {
        Err(e) => {
            eprint!("Error occurred when trying to access {file_path}: {e}");
            return None;
        }
        Ok(x) => x,
    };

    /* Iterate over contents and recursively process each. */
    let mut dir_results: Vec<_> = Vec::new();
    for dir_entry in dir_iter {
        let Ok(dir) = dir_entry else {
            continue;
        };

        let Some(path_str) = canonical_path_str(dir) else {
            continue;
        };

        if let Some(mut result) = search_dir(query, path_str.to_string(), &flags) {
            dir_results.append(&mut result);
        }
    }

    return Some(dir_results);
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
) -> Option<FileMatches> {
    if query.is_empty() {
        return None;
    }

    let mut query_copy = query.clone(); // Don't want to affect the original query string
    if flags.case_insensitive {
        query_copy.make_ascii_lowercase();
    }

    let mut matched_lines: Vec<MatchedLine> = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        let mut match_locations = Vec::new();

        let mut line_copy = line.to_string(); // Same as above but for the buffer contents
        if flags.case_insensitive {
            line_copy.make_ascii_lowercase();
        }

        let mut start = 0;
        while let Some(index) = line_copy[start..].find(&query_copy) {
            let idx = start + index;
            match_locations.push(idx);
            start = idx + 1;
        }

        if !match_locations.is_empty() {
            matched_lines.push(MatchedLine {
                line: line.to_string(),
                line_number: line_number as u32,
                locations: match_locations,
            });
        }
    }

    if matched_lines.is_empty() {
        return None;
    } else {
        return Some(FileMatches {
            file_path: file_path.to_string(),
            matches: matched_lines,
        });
    }
}
