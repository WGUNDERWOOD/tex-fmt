use crate::comments::*;

const WRAP: usize = 80;

pub fn needs_wrap(file: &str) -> bool {
    file.lines().any(|l| l.len() > WRAP)
}

pub fn line_needs_wrap(line: &str) -> bool {
    line.len() > WRAP
}

pub fn find_wrap_point(line: &str) -> Option<usize> {
    let mut wrap_point: Option<usize> = None;
    let mut after_non_space = false;
    for i in 0..WRAP {
        let char_is_space: bool = line.chars().nth(i) == Some(' ');
        if char_is_space {
            if after_non_space {
                wrap_point = Some(i);
            }
        } else {
            after_non_space = true;
        }
    }
    wrap_point
}

pub fn wrap_line(line: &str) -> String {
    let mut remaining_line = line.to_string();
    let mut new_line = "".to_string();
    let mut can_wrap = true;
    while line_needs_wrap(&remaining_line) && can_wrap {
        let wrap_point = find_wrap_point(&remaining_line);
        let comm = find_comment(&remaining_line);
        match wrap_point {
            Some(p) => {
                let line_start = match comm {
                    Some(ref c) => {
                        if p > c.idx {
                            "%"
                        }
                        else {
                            ""
                        }
                    }
                    None => ""
                };
                new_line.push_str(&remaining_line[0..p]);
                new_line.push('\n');
                remaining_line = remaining_line[p..remaining_line.len()].to_string();
                remaining_line.insert_str(0, line_start);
            },
            None => {
                can_wrap = false;
                println!("long line cannot be wrapped!");
                println!("{}", line);
            }
        }
    }
    new_line.push_str(&remaining_line);
    new_line
}

pub fn wrap(file: &str) -> String {
    let mut new_file = "".to_string();
    let lines: Vec<&str> = file.lines().collect();
    for line in lines {
        if line_needs_wrap(line) {
            let new_line = wrap_line(line);
            new_file.push_str(&new_line);
        } else {
            new_file.push_str(line);
        }
        new_file.push('\n');
    }
    new_file
}

#[cfg(test)]

#[test]
fn test_wrap_line() {
    // no comment
    let s_in = "This line is too long because it has more than eighty characters inside it. \
        Therefore it should be split.";
    let s_out = "This line is too long because it has more than eighty characters inside it.\n \
        Therefore it should be split.";
    assert_eq!(wrap_line(s_in), s_out);
    // break before comment
    let s_in = "This line is too long because it has more than eighty characters inside it. \
        Therefore it % should be split.";
    let s_out = "This line is too long because it has more than eighty characters inside it.\n \
        Therefore it % should be split.";
    assert_eq!(wrap_line(s_in), s_out);
    // break after comment
    let s_in = "This line is too long because % it has more than eighty characters inside it. \
        Therefore it should be split.";
    let s_out = "This line is too long because % it has more than eighty characters inside it.\n\
        % Therefore it should be split.";
    assert_eq!(wrap_line(s_in), s_out);
    // leading spaces
    let s_in = "    Thislineistoolongbecauseithasmorethaneightycharactersinsideiteventhoughitstartswithspaces. \
        Thereforeitshouldbesplit.";
    let s_out = s_in;
    assert_eq!(wrap_line(s_in), s_out);
}
