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

pub fn begin_end_environments_new_line(file: &str) -> String {
    let mut new_file = "".to_string();
    let lines: Vec<&str> = file.lines().collect();
    for line in lines.iter() {
        if RE_ENV_BEGIN.is_match(line) || RE_ENV_END.is_match(line) {
            let comm = find_comment(line);
            let comment = get_comment(line, &comm);
            let text = remove_comment(line, &comm);
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
