//! Utilities for wrapping long lines

use crate::args::Args;
use crate::comments::find_comment_index;
use crate::format::{Pattern, State};
use crate::logging::{record_line_log, Log};
use crate::regexes::VERB;
use log::Level;
use log::LevelFilter;

/// String slice to start wrapped text lines
pub const TEXT_LINE_START: &str = "";
/// String slice to start wrapped comment lines
pub const COMMENT_LINE_START: &str = "% ";

/// Check if a line needs wrapping
#[must_use]
pub fn needs_wrap(line: &str, indent_length: usize, args: &Args) -> bool {
    args.wrap && (line.chars().count() + indent_length > args.wraplen.into())
}

/// Find the best place to break a long line
fn find_wrap_point(
    line: &str,
    indent_length: usize,
    args: &Args,
    pattern: &Pattern,
) -> Option<usize> {
    let mut wrap_point: Option<usize> = None;
    let mut after_non_percent = false;
    let mut prev_char: Option<char> = None;

    // TODO better way to calculate this using i index?
    // I think this is actually wrong, need line.len() as using byte indices

    // TODO wrap_point should be last byte before the split is inserted
    // This may not be a valid code point
    // TODO Rewrite all this logic

    let wrap_boundary = usize::from(args.wrapmin) - indent_length;

    if pattern.contains_verb && line.contains(VERB) {
        // Special wrapping for lines containing \verb|...|
        let verb_start = line.find(VERB).unwrap();
        let verb_end = line[verb_start + 6..].find('|').unwrap_or(verb_start)
            + verb_start
            + 6;
        if verb_start == 0 {
            after_non_percent = true;
        }
        for (i, c) in line.char_indices() {
            let inside_verb = (verb_start <= i) && (i <= verb_end);
            if i >= wrap_boundary && wrap_point.is_some() {
                break;
            }
        // TODO make this faster if only one wrap char provided
        if args.wrap_chars.contains(&c) && prev_char != Some('\\') && !inside_verb {
                if after_non_percent {
                    wrap_point = Some(i);
                }
            } else if c != '%' {
                after_non_percent = true;
            }
            prev_char = Some(c);
        }
    } else {
        // Wrapping for lines not containing \verb|...|
        for (i, c) in line.char_indices() {
            if i >= wrap_boundary && wrap_point.is_some() {
                break;
            }
            if args.wrap_chars.contains(&c) && prev_char != Some('\\') {
                if after_non_percent {
                    wrap_point = Some(i);
                }
            } else if c != '%' {
                after_non_percent = true;
            }
            prev_char = Some(c);
        }
    }

    // Return *byte* index rather than *char* index.
    let last_char_index = line.char_indices().rev().next().map(|(i, _)| i).unwrap();
    match wrap_point {
        Some(p) if p < last_char_index => Some(p),
        _ => None,
    }
}

/// Wrap a long line into a short prefix and a suffix
pub fn apply_wrap<'a>(
    line: &'a str,
    indent_length: usize,
    state: &State,
    file: &str,
    args: &Args,
    logs: &mut Vec<Log>,
    pattern: &Pattern,
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
    let wrap_point = find_wrap_point(line, indent_length, args, pattern);
    let comment_index = find_comment_index(line, pattern);

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
    }

    wrap_point.map(|p| {
        let this_line = &line[0..=p];
        let next_line_start = comment_index.map_or("", |c| {
            if p > c {
                COMMENT_LINE_START
            } else {
                TEXT_LINE_START
            }
        });
        let next_line = &line[p+1..];
        [this_line, next_line_start, next_line]
    })
}
