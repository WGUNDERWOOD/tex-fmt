//! Utilities for finding, extracting and removing LaTeX comments

/// Find the location where a comment begins in a line
pub fn find_comment_index(line: &str) -> Option<usize> {
    let mut prev_c = ' ';
    for (i, c) in line.char_indices() {
        if c == '%' && prev_c != '\\' {
            return Some(i);
        }
        prev_c = c;
    }
    None
}

/// Remove a comment from the end of a line
pub fn remove_comment(line: &str, comment: Option<usize>) -> &str {
    comment.map_or_else(|| line, |c| &line[0..c])
}
