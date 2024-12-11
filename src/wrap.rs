//! Utilities for wrapping long lines

use crate::args::*;
use crate::comments::*;
use crate::format::*;
use crate::logging::*;
use crate::regexes::RE_SPLITTING;
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

    let mut reached_boundary = false;

    // Return *byte* index rather than *char* index.
    for (i, c) in line.char_indices() {
        line_width += 1;
        if line_width > wrap_boundary && !wrap_points.is_empty() {
            reached_boundary = true;
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
    if !reached_boundary {
        wrap_points.push(line_width);
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

/// Decides whether two contiguous lines can be re-wrapped. This assumes that
/// `current_line` has already been checked to not be longer than
/// `args.wraplen`.
pub fn can_rewrap(
    current_line: &str,
    next_line: Option<&str>,
    indent_length: usize,
    args: &Args,
) -> Option<usize> {
    // Early return checks
    if
    // If we don't wrap, are on an empty line, or there is no next line,
    !args.wrap || current_line.is_empty() || next_line.is_none()
    // or if the current line starts with a splitting command,
    || RE_SPLITTING.is_match(current_line)
    // or if the current line contains a comment,
    || find_comment_index(current_line).is_some()
    {
        return None;
    }

    // Doesn't panic because None causes early return
    let next_line: &str = next_line.unwrap().trim_start();

    if next_line.is_empty()
    // Re-wrapping comes after splitting, so if `next_line` contains a splitting
    // command, then it's at the start, and it shouldn't be rewrapped to the
    // previous line
    || RE_SPLITTING.is_match(next_line)
    {
        return None;
    }

    // Compute once because `.count()` is O(n)
    let current_line_length = current_line.chars().count() + indent_length;

    // Sanity check that the current line is short enough
    debug_assert!(current_line_length <= args.wraplen.into());

    // Previous line ensures `next_line` is trimmed. 0 is passed for
    // `indent_length` because we mostly care about the wrap points at the start
    // of the line
    let Some(candidate_rewrap_points) = find_wrap_points(next_line, 0, args)
    else {
        // Early return if the next line does not contain any wrap points
        return None;
    };

    // Get an optional comment index from the next line
    let maybe_comment_index = find_comment_index(next_line);

    let mut rewrap_point = None;
    for candidate_point in candidate_rewrap_points {
        // If the next line contains a comment, stop considering re-wrap points
        // later than the comment index.
        if let Some(comment_index) = maybe_comment_index {
            if candidate_point >= comment_index {
                break;
            }
        }

        let candidate_length =
            current_line_length + next_line[0..candidate_point].chars().count();

        if candidate_length + indent_length <= args.wrapmin.into() {
            rewrap_point = Some(candidate_point);
        }
    }

    rewrap_point
}
