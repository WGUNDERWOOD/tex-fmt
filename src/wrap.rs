use crate::comments::*;
use crate::ignore::*;
use crate::logging::*;
use crate::parse::*;
use crate::regexes::*;
use log::Level::{Warn, Info};

const WRAP: usize = 80;

pub fn needs_wrap(file: &str) -> bool {
    file.lines().any(|l| l.len() > WRAP)
}

fn find_wrap_point(line: &str) -> Option<usize> {
    let mut wrap_point: Option<usize> = None;
    let mut after_char = false;
    let mut prev_char: Option<char> = None;
    for i in 0..WRAP {
        if line.chars().nth(i) == Some(' ') && prev_char != Some('\\') {
            if after_char {
                wrap_point = Some(i);
            }
        } else if line.chars().nth(i) != Some('%') {
            after_char = true;
        }
        prev_char = line.chars().nth(i)
    }
    wrap_point
}

fn wrap_line(
    line: &str,
    linum: usize,
    args: &Cli,
    logs: &mut Vec<Log>,
    pass: Option<usize>,
    filename: &str,
) -> String {
    if args.verbose {
        record_log(
            logs,
            Info,
            pass,
            filename.to_string(),
            Some(linum),
            Some(line.to_string()),
            "Wrapping long line.".to_string(),
        );
    }
    let mut remaining_line = line.to_string();
    let mut new_line = "".to_string();
    let mut can_wrap = true;
    while needs_wrap(&remaining_line) && can_wrap {
        let wrap_point = find_wrap_point(&remaining_line);
        let comment_index = find_comment_index(&remaining_line);
        match wrap_point {
            Some(p) => {
                let line_start = match comment_index {
                    Some(c) => {
                        if p > c {
                            "%"
                        } else {
                            ""
                        }
                    }
                    None => "",
                };
                new_line.push_str(&remaining_line[0..p]);
                new_line.push('\n');
                remaining_line =
                    remaining_line[p..remaining_line.len()].to_string();
                remaining_line.insert_str(0, line_start);
            }
            None => {
                can_wrap = false;
            }
        }
    }
    new_line.push_str(&remaining_line);
    new_line
}

pub fn wrap(
    file: &str,
    filename: &str,
    logs: &mut Vec<Log>,
    pass: Option<usize>,
    args: &Cli,
) -> String {
    if args.verbose {
        record_log(
            logs,
            Info,
            pass,
            filename.to_string(),
            None,
            None,
            format!("Wrap pass {}.", pass.unwrap()),
        );
    }
    let mut new_file = "".to_string();
    let mut verbatim_count = 0;
    let mut ignore = Ignore::new();
    for (linum, line) in file.lines().enumerate() {
        if RE_VERBATIM_END.is_match(line) {
            verbatim_count -= 1;
        }
        ignore = get_ignore(line, linum, ignore, filename, logs, pass, false);
        if needs_wrap(line) && verbatim_count == 0 && !is_ignored(&ignore) {
            let new_line = wrap_line(line, linum, args, logs, pass, filename);
            new_file.push_str(&new_line);
            if needs_wrap(&new_line) && !is_ignored(&ignore) {
                record_log(
                    logs,
                    Warn,
                    pass,
                    filename.to_string(),
                    Some(linum),
                    Some(new_line),
                    "Line cannot be wrapped:".to_string(),
                );
            }
        } else {
            new_file.push_str(line);
            if needs_wrap(line) && !is_ignored(&ignore) {
                record_log(
                    logs,
                    Warn,
                    pass,
                    filename.to_string(),
                    Some(linum),
                    Some(line.to_string()),
                    "Line cannot be wrapped:".to_string(),
                );
            }
        }
        new_file.push('\n');
        if RE_VERBATIM_BEGIN.is_match(line) {
            verbatim_count += 1;
        }
    }

    new_file
}
