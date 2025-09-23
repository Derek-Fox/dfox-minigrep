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
    /* First check if we have a file. If so, try to read its contents and get matches. Base case of recursion. */
    match fs::metadata(&file_path) {
        Err(e) => {
            eprintln!("Error occured when trying to access metadata of {file_path}: {e}");
            return Vec::new();
        }
        Ok(metadata) => {
            if metadata.is_file() {
                let contents = match fs::read_to_string(&file_path) {
                    Err(_) => return Vec::new(), //file doesn't contain valid unicode -> happens all the time, don't need to warn user about that
                    Ok(x) => x,
                };
                if let Some(matches) = search_contents(query, contents, &file_path, flags) {
                    return vec![matches];
                } else {
                    return Vec::new();
                }
            }
        }
    }

    let dir_iter = match fs::read_dir(&file_path) {
        Err(e) => {
            eprint!("Error occurred when trying to access {file_path}: {e}");
            return Vec::new();
        }
        Ok(iter) => iter,
    };

    let mut dir_results: Vec<_> = Vec::new();
    for dir_entry in dir_iter {
        let canon_path = match dir_entry {
            Err(_e) => {
                eprintln!("Failed to retrieve next entry from OS. (idk man)");
                continue;
            }
            Ok(dir) => match dir.path().canonicalize() {
                Err(e) => {
                    eprintln!("Error occurred when trying to get the path of {dir:#?}: {e}");
                    continue;
                }
                Ok(path) => path,
            },
        };
        let path_str = match canon_path.to_str() {
            None => {
                eprintln!("Error, {canon_path:#?} contains invalid unicode.");
                continue;
            }
            Some(s) => s,
        };

        let mut result = search_dir(query, path_str.to_string(), &flags);
        if !result.is_empty() {
            dir_results.append(&mut result);
        }
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
) -> Option<FileMatches> {
    if query.is_empty() {
        return None;
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

    if matched_lines.is_empty() {
        return None;
    } else {
        return Some(FileMatches {
            file_path: file_path.to_string(),
            matches: matched_lines,
        });
    }
}
