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
    // TODO this should be done with bytes too?
    // Or maybe everything should use char length not byte length?
    args.wrap && (line.chars().count() + indent_length > args.wraplen.into())
}

fn is_wrap_point(
    i: usize,
    c: char,
    prev_c: Option<char>,
    inside_verb: bool,
    line_len: usize,
    args: &Args,
) -> bool {
    // Character c must be a valid wrapping character
    args.wrap_chars.contains(&c)
        // Must not be preceded by '\'
        && prev_c != Some('\\')
        // Do not break inside a \verb|...|
        && !inside_verb
        // No point breaking at the end of the line
        && (i + 1 < line_len)
}

fn get_verb_end(verb_start: Option<usize>, line: &str) -> Option<usize> {
    let verb_len = 6;
    verb_start
        .map(|v| line[v + verb_len..].find('|').unwrap_or(v) + v + verb_len)
}

fn is_inside_verb(
    i: usize,
    contains_verb: bool,
    verb_start: Option<usize>,
    verb_end: Option<usize>,
) -> bool {
    if contains_verb {
        (verb_start.unwrap() <= i) && (i <= verb_end.unwrap())
    } else {
        false
    }
}

/// Find the best place to break a long line.
/// Provided as a *byte* index, not a *char* index.
fn find_wrap_point(
    line: &str,
    indent_length: usize,
    args: &Args,
    pattern: &Pattern,
) -> Option<usize> {
    let mut wrap_point: Option<usize> = None;
    let mut prev_c: Option<char> = None;
    let contains_verb = pattern.contains_verb && line.contains(VERB);
    let verb_start: Option<usize> =
        contains_verb.then(|| line.find(VERB).unwrap());
    let verb_end = get_verb_end(verb_start, line);
    let mut after_non_percent = verb_start == Some(0);
    let wrap_boundary = usize::from(args.wrapmin) - indent_length;
    let line_len = line.len();

    for (i, c) in line.char_indices() {
        if i >= wrap_boundary && wrap_point.is_some() {
            break;
        }
        // Special wrapping for lines containing \verb|...|
        let inside_verb =
            is_inside_verb(i, contains_verb, verb_start, verb_end);
        if is_wrap_point(i, c, prev_c, inside_verb, line_len, args) {
            if after_non_percent {
                // Get index of the byte after which
                // line break will be inserted.
                // Note this may not be a valid char index.
                wrap_point = Some(i + c.len_utf8() - 1);
            }
        } else if c != '%' {
            after_non_percent = true;
        }
        prev_c = Some(c);
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
        let next_line = &line[p + 1..];
        [this_line, next_line_start, next_line]
    })
}
