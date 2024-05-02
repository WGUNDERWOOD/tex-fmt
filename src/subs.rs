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

// TODO
//pub fn begin_end_environments_new_line(file: &str) -> String {
    //file
        //.lines()
        //.map(|l| remove_comment(l))
        //.map(|l|
             //RE_ENV_BEGIN_SHARED_LINE
             //.replace_all(&l, "$prev\n$env")
             //.to_string())
        //.map(|l|
             //RE_ENV_END_SHARED_LINE
             //.replace_all(&l, "$prev\n$env")
             //.to_string())
        //.fold(String::new(), |a, b| a + &b + "\n")

    //let lines: Vec<&str> = new_file.lines().collect();
    //let n_lines = lines.len();
    //let mut new_lines = vec![];
    //for i in 0..n_lines {
        //let line = lines[i];
        //if RE_ENV_BEGIN_SHARED_LINE
    //}
    //RE_ENV_BEGIN_SHARED_LINE
        //.replace_all(file, "$prev\n$env")
        //.to_string()
//}

pub fn remove_comment(line: &str) -> String {
    RE_COMMENT.replace_all(line, "$text").to_string()
}
