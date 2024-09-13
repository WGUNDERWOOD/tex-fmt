//! Utilities for finding, extracting and removing LaTeX comments

/// Find the location where a comment begins in a line
pub fn find_comment_index(line: &str) -> Option<usize> {
    let chars = line.chars();
    let mut prev_c = ' ';
    for (i, c) in chars.enumerate() {
        if c == '%' && prev_c != '\\' {
            return Some(i);
        }
        prev_c = c;
    }
    None
}

/// Remove a comment from the end of a line
pub fn remove_comment(line: &str, comment: Option<usize>) -> String {
    comment.map_or_else(|| line.to_string(), |c| line.chars().take(c).collect())
}

/// Extract a comment from the end of a line
pub fn get_comment(line: &str, comment: Option<usize>) -> String {
    comment.map_or_else(String::new, |c| line.chars().skip(c).collect())
}
