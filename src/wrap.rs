use crate::comments::*;
use crate::ignore::*;
use crate::leave::*;
use crate::logging::*;
use crate::parse::*;
use log::Level::{Info, Warn};

const WRAP: usize = 80;

pub fn needs_wrap(text: &str) -> bool {
    text.lines().any(|l| l.chars().count() > WRAP)
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
    file: &str,
) -> String {
    if args.verbose {
        record_log(
            logs,
            Info,
            pass,
            file.to_string(),
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
                let first_segment: String =
                    remaining_line.chars().take(p).collect();
                new_line.push_str(&first_segment);
                new_line.push('\n');
                remaining_line = remaining_line.chars().skip(p).collect();
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
    text: &str,
    file: &str,
    logs: &mut Vec<Log>,
    pass: Option<usize>,
    args: &Cli,
) -> String {
    if args.verbose {
        record_log(
            logs,
            Info,
            pass,
            file.to_string(),
            None,
            None,
            format!("Wrap on pass {}.", pass.unwrap_or_default()),
        );
    }
    let mut new_text = "".to_string();
    let mut ignore = Ignore::new();
    let mut leave = Leave::new();
    for (linum, line) in text.lines().enumerate() {
        ignore = get_ignore(line, linum, ignore, file, logs, pass, false);
        leave = get_leave(line, linum, leave, file, logs, pass, false);
        if needs_wrap(line) && !leave.visual && !ignore.visual {
            let new_line = wrap_line(line, linum, args, logs, pass, file);
            new_text.push_str(&new_line);
            if needs_wrap(&new_line) && !ignore.visual {
                record_log(
                    logs,
                    Warn,
                    pass,
                    file.to_string(),
                    Some(linum),
                    Some(new_line),
                    "Line cannot be wrapped:".to_string(),
                );
            }
        } else {
            new_text.push_str(line);
            if needs_wrap(line) && !ignore.visual {
                record_log(
                    logs,
                    Warn,
                    pass,
                    file.to_string(),
                    Some(linum),
                    Some(line.to_string()),
                    "Line cannot be wrapped:".to_string(),
                );
            }
        }
        new_text.push('\n');
    }

    new_text
}
