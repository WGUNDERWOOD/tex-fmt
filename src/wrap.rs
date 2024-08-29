use crate::comments::*;
use crate::format::*;
use crate::ignore::*;
use crate::leave::*;
use crate::logging::*;
use crate::parse::*;
use log::Level::{Info, Trace, Warn};

const WRAP_MIN: usize = 70;
const WRAP_MAX: usize = 80;

pub fn needs_wrap(line: &str, state: &State, logs: &mut Vec<Log>) -> bool {
    let ignore = get_ignore(line, &state, logs, false);
    let leave = get_leave(line, &state);
    (line.chars().count() > WRAP_MAX) && !leave.visual && !ignore.visual
}

fn find_wrap_point(line: &str) -> Option<usize> {
    let mut wrap_point: Option<usize> = None;
    let mut after_char = false;
    let mut prev_char: Option<char> = None;
    for (i, c) in line.chars().enumerate() {
        if i >= WRAP_MIN && wrap_point.is_some() {
            break;
        }
        if c == ' ' && prev_char != Some('\\') {
            if after_char {
                wrap_point = Some(i);
            }
        } else if c != '%' {
            after_char = true;
        }
        prev_char = Some(c)
    }
    wrap_point
}

pub fn apply_wrap(
    line: &str,
    linum_new: usize,
    linum_old: usize,
    state: &State,
    file: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> Option<(String, String)> {
    if args.trace {
        record_line_log(
            logs,
            Trace,
            file,
            linum_new,
            linum_old,
            line,
            "Wrapping long line.",
        );
    }
    let wrap_point = find_wrap_point(&line);
    let comment_index = find_comment_index(&line);
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
            let line_1 = line.chars().take(p).collect();
            let mut line_2: String = line.chars().skip(p).collect();
            line_2.insert_str(0, line_start);
            Some((line_1, line_2))
        }
        None => {
            record_line_log(
                logs,
                Warn,
                file,
                linum_new,
                linum_old,
                line,
                "Line cannot be wrapped.",
            );
            None
        }
    }
}
