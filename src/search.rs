use rayon::prelude::*;
use std::{borrow::Cow, fs, path::{Path, PathBuf}};

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
    pub(crate) case_insensitive: bool,
}

/**
 * Recursively search directory. Searches all files in the directory, and searches any/all subdirectories.
 */
pub(crate) fn search_dir(
    query: &str,
    path: &Path,
    flags: &SearchFlags,
) -> Option<Vec<FileMatches>> {
    /*  Can't access file -> silently fail */
    let Ok(metadata) = path.metadata() else {
        return None;
    };

    if metadata.is_symlink() {
        return None;
    }

    /* First check if we have a file. If so, try to read its contents and get matches. Base case of recursion. */
    if metadata.is_file() {
        match search_contents(query, path, flags) {
            Some(matches) => return Some(vec![matches]),
            None => return None,
        };
    }

    /* Otherwise, we have directory. Get an iterator over contents of directory. */
    let Ok(dir_iter) = fs::read_dir(&path) else {
        return None;
    };

    /* Collect directory items' paths to a vec */
    let entries: Vec<PathBuf> = dir_iter
        .filter_map(|dir_entry| dir_entry.ok().map(|e| e.path()))
        .collect();

    /* Iterate over paths and process each in a new thread using rayon threadpool. */
    let dir_results: Vec<FileMatches> = entries
        .par_iter() // rayon threadpool iterator
        .filter_map(|entry_path| search_dir(query, entry_path, flags)) // recursively call for each subdir, throw away Nones
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
    query: &str,
    file_path: &Path,
    flags: &SearchFlags,
) -> Option<FileMatches> {
    let Ok(contents) = fs::read_to_string(file_path) else {
        return None; // file doesn't contain valid UTF-8 - just ignore it
    };

    let query_copy = if flags.case_insensitive {
        Cow::Owned(query.to_ascii_lowercase())
    } else {
        Cow::Borrowed(query)
    };

    let mut matched_lines: Vec<MatchedLine> = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        let mut match_locations = Vec::new();

        let line_copy = if flags.case_insensitive {
            Cow::Owned(line.to_ascii_lowercase())
        } else {
            Cow::Borrowed(line)
        };

        let mut start = 0;
        while let Some(index) = line_copy[start..].find(query_copy.as_ref()) {
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
        file_path: file_path.into(),
        matches: matched_lines,
    })
}
