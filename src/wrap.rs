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
    args.wrap && (line.chars().count() + indent_length > args.wraplen.into())
}

/// Returns a list of possible break points in the given line, taking into
/// account the length of indentation that will be added to the line. Returns
/// `None` is no wrap points are found, so it should never return an empty list.
fn find_wrap_points(
    line: &str,
    indent_length: usize,
    args: &Args,
) -> Option<Vec<usize>> {
    let mut wrap_points: Vec<usize> = Vec::new();
    let mut after_char = false;
    let mut prev_char: Option<char> = None;

    let mut line_width = 0;

    let wrap_boundary = usize::from(args.wrapmin) - indent_length;

    // Return *byte* index rather than *char* index.
    for (i, c) in line.char_indices() {
        line_width += 1;
        if line_width > wrap_boundary && !wrap_points.is_empty() {
            break;
        }
        if c == ' ' && prev_char != Some('\\') {
            if after_char {
                wrap_points.push(i);
            }
        } else if c != '%' {
            after_char = true;
        }
        prev_char = Some(c);
    }

    if wrap_points.is_empty() {
        return None;
    }

    Some(wrap_points)
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
    // The `unwrap()` doesn't panic because find_wrap_points() returns None if
    // there are no wrap points
    let wrap_point = find_wrap_points(line, indent_length, args)
        .map(|list| list.last().copied().unwrap());
    let comment_index = find_comment_index(line);

    match wrap_point {
        Some(p) if p <= args.wraplen.into() => {}
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
