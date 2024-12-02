//! Utilities for wrapping long lines

use crate::args::*;
use crate::comments::*;
use crate::format::*;
use crate::logging::*;
use log::Level;
use log::LevelFilter;

/// String slice to start wrapped text lines
pub const TEXT_LINE_START: &str = "";
/// String slice to start wrapped comment lines
pub const COMMENT_LINE_START: &str = "% ";

/// Check if a line needs wrapping
pub fn needs_wrap(line: &str, indent_length: usize, args: &Args) -> bool {
    args.wrap && (line.chars().count() + indent_length > args.wrap.into())
}

/// Find the best place to break a long line
fn find_wrap_point(
    line: &str,
    indent_length: usize,
    args: &Args,
) -> Option<usize> {
    let mut wrap_point: Option<usize> = None;
    let mut after_char = false;
    let mut prev_char: Option<char> = None;

    let mut line_width = 0;

    let wrap_boundary = usize::from(args.wrapmin) - indent_length;

    // Return *byte* index rather than *char* index.
    for (i, c) in line.char_indices() {
        line_width += 1;
        if line_width > wrap_boundary && wrap_point.is_some() {
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

/// Wrap a long line into a short prefix and a suffix
pub fn apply_wrap<'a>(
    line: &'a str,
    indent_length: usize,
    state: &State,
    file: &str,
    args: &Args,
    logs: &mut Vec<Log>,
) -> Option<[&'a str; 3]> {
    if args.verbosity == LevelFilter::Trace {
        record_line_log(
            logs,
            Level::Trace,
            file,
            state.linum_new,
            state.linum_old,
            line,
            "Wrapping long line.",
        );
    }
    let wrap_point = find_wrap_point(line, indent_length, args);
    let comment_index = find_comment_index(line);

    match wrap_point {
        Some(p) if p <= args.wrap.into() => {}
        _ => {
            record_line_log(
                logs,
                Level::Warn,
                file,
                state.linum_new,
                state.linum_old,
                line,
                "Line cannot be wrapped.",
            );
        }
    };

    wrap_point.map(|p| {
        let this_line = &line[0..p];
        let next_line_start = comment_index.map_or("", |c| {
            if p > c {
                COMMENT_LINE_START
            } else {
                TEXT_LINE_START
            }
        });
        let next_line = &line[p + 1..];
        [this_line, next_line_start, next_line]
    })
}
