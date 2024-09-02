use crate::comments::*;
use crate::format::*;
use crate::logging::*;
use crate::parse::*;
use log::Level::{Trace, Warn};

const WRAP_MIN: usize = 70;
const WRAP_MAX: usize = 80;

pub fn needs_wrap(line: &str, state: &State, args: &Cli) -> bool {
    !args.keep
        && !state.verbatim.visual
        && !state.ignore.visual
        && (line.chars().count() > WRAP_MAX)
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
        prev_char = Some(c);
    }
    wrap_point
}

pub fn apply_wrap(
    line: &str,
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
            state.linum_new,
            state.linum_old,
            line,
            "Wrapping long line.",
        );
    }
    let wrap_point = find_wrap_point(line);
    let comment_index = find_comment_index(line);

    match wrap_point {
        Some(p) if p <= WRAP_MAX => {}
        _ => {
            record_line_log(
                logs,
                Warn,
                file,
                state.linum_new,
                state.linum_old,
                line,
                "Line cannot be wrapped.",
            );
        }
    };

    wrap_point.map(|p| {
        let line_start =
            comment_index.map_or("", |c| if p > c { "%" } else { "" });
        let mut line_1: String = line.chars().take(p).collect();
        line_1 = line_1.trim_end().to_string();
        let mut line_2: String = line.chars().skip(p).collect();
        line_2.insert_str(0, line_start);
        (line_1, line_2)
    })
}
