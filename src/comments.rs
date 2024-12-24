//! Utilities for finding, extracting and removing LaTeX comments

/// Find the location where a comment begins in a line
pub fn find_comment_index(line: &str) -> Option<usize> {
    // often there is no '%' so check this first
    // TODO put this into a pattern check?
    if line.contains('%') {
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

pub fn comment_spacing(line: &str) -> String {
    let comment_index = find_comment_index(line);
    if comment_index.is_some()
        && line.len() > comment_index.unwrap() + 1
            && !([b' ', b'%'].contains(
            &line.as_bytes()[comment_index.unwrap() + 1])){
            let mut new_line = line.to_owned();
            new_line.insert(comment_index.unwrap() + 1, ' ');
            return new_line;
        }
    line.to_string()
}

/// Remove a comment from the end of a line
pub fn remove_comment(line: &str, comment: Option<usize>) -> &str {
    comment.map_or_else(|| line, |c| &line[0..c])
}
