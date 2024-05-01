use crate::TAB;
use crate::regexes::*;

pub fn remove_extra_newlines(file: &str) -> String {
    RE_NEWLINES.replace_all(file, "\n\n").to_string()
}

pub fn remove_tabs(file: &str) -> String {
    let replace = (0..TAB).map(|_| " ").collect::<String>();
    RE_TABS.replace_all(file, replace).to_string()
}

pub fn remove_trailing_spaces(file: &str) -> String {
    RE_TRAIL.replace_all(file, "\n").to_string()
}

pub fn remove_comment(line: &str) -> String {
    let new_line = RE_PERCENT.replace_all(line, "").to_string();
    RE_COMMENT.replace_all(&new_line, "").to_string()
}
