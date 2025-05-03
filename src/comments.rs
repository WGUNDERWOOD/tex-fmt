//! Utilities for finding, extracting and removing LaTeX comments

use crate::format::Pattern;

/// Find the location where a comment begins in a line
#[must_use]
pub fn find_comment_index(line: &str, pattern: &Pattern) -> Option<usize> {
    // often there is no '%' so check this first
    if pattern.contains_comment {
        let mut prev_c = ' ';
        for (i, c) in line.char_indices() {
            if c == '%' && prev_c != '\\' {
                return Some(i);
            }
            prev_c = c;
        }
    }
    None
}

/// Remove a comment from the end of a line
#[must_use]
pub fn remove_comment(line: &str, comment: Option<usize>) -> &str {
    comment.map_or_else(|| line, |c| &line[0..c])
}
