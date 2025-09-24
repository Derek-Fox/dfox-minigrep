use std::{fs, path::PathBuf};

pub(crate) struct FileMatches {
    pub(crate) file_path: PathBuf,
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
 * Check metadata of path to see if it is a file. Returns false on failure to access metadata.
 */
fn is_file(path: &PathBuf) -> bool {
    match fs::metadata(&path) {
        Err(e) => {
            eprintln!(
                "Error occured when trying to access metadata of {}: {e}",
                path.display()
            );
            return false;
        }
        Ok(metadata) => metadata.is_file(),
    }
}

/**
 * Recursively search directory. Searches all files in the directory, and searches any/all subdirectories.
 */
pub(crate) fn search_dir(
    query: &String,
    path: &PathBuf,
    flags: &SearchFlags,
) -> Option<Vec<FileMatches>> {
    /* First check if we have a file. If so, try to read its contents and get matches. Base case of recursion. */
    if is_file(path) {
        if let Some(matches) = search_contents(query, path, flags) {
            return Some(vec![matches]);
        } else {
            return None;
        }
    }

    /* Get an iterator over contents of directory */
    let dir_iter = match fs::read_dir(&path) {
        Err(e) => {
            eprintln!(
                "Error occurred when trying to access {}: {e}",
                path.display()
            );
            return None;
        }
        Ok(x) => x,
    };

    /* Iterate over contents and recursively process each. */
    let dir_results: Vec<FileMatches> = dir_iter
        .filter_map(|dir_entry| dir_entry.ok()) // Only get Err for OS issue - just ignore
        .filter_map(|dir| search_dir(query, &dir.path(), flags)) // recursively call for each subdir, throw away Nones
        .flatten() // Vec<Vec<FileMatches>> -> Vec<FileMatches>
        .collect();

    if dir_results.is_empty() {
        return None;
    }

    Some(dir_results)
}

/**
 * Search contents for instances of query. Returns a list of MatchedLine structs which capture the line
 * and information about the location of the match.
 */
pub(crate) fn search_contents(
    query: &String,
    file_path: &PathBuf,
    flags: &SearchFlags,
) -> Option<FileMatches> {
    let Ok(contents) = fs::read_to_string(file_path) else {
        return None; // file doesn't contain valid UTF-8 - just ignore it
    };

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
                line_number: line_number as u32 + 1, // lines start at 1
                locations: match_locations,
            });
        }
    }

    if matched_lines.is_empty() {
        return None;
    }

    Some(FileMatches {
        file_path: file_path.to_path_buf(),
        matches: matched_lines,
    })
}
