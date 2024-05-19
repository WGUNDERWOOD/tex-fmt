use crate::comments::*;
use crate::regexes::*;
use crate::TAB;

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

// TODO these should be abstracted together
pub fn begin_end_environments_new_line(file: &str) -> String {
    let mut new_file = "".to_string();
    for line in file.lines() {
        if RE_ENV_BEGIN.is_match(line) || RE_ENV_END.is_match(line) {
            let comment_index = find_comment_index(line);
            let comment = get_comment(line, comment_index);
            let text = remove_comment(line, comment_index);
            let text = &RE_ENV_BEGIN_SHARED_LINE
                .replace_all(text, "$prev\n$env")
                .to_string();
            let text = &RE_ENV_END_SHARED_LINE
                .replace_all(text, "$prev\n$env")
                .to_string();
            new_file.push_str(text);
            new_file.push_str(comment);
        } else {
            new_file.push_str(line);
        }
        new_file.push('\n');
    }
    new_file
}

pub fn items_new_line(file: &str) -> String {
    let mut new_file = "".to_string();
    for line in file.lines() {
        if RE_ITEM.is_match(line) {
            let comment_index = find_comment_index(line);
            let comment = get_comment(line, comment_index);
            let text = remove_comment(line, comment_index);
            let text = &RE_ITEM_SHARED_LINE
                .replace_all(text, "$prev\n$env")
                .to_string();
            new_file.push_str(text);
            new_file.push_str(comment);
        } else {
            new_file.push_str(line);
        }
        new_file.push('\n');
    }
    new_file
}
