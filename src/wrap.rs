//! Utilities for wrapping long lines

use crate::args::Args;
use crate::comments::find_comment_index;
use crate::format::{Pattern, State};
use crate::logging::{record_line_log, Log};
use crate::regexes::VERBS;
use log::Level;
use log::LevelFilter;
use std::path::Path;

/// String slice to start wrapped text lines
pub const TEXT_LINE_START: &str = "";
/// String slice to start wrapped comment lines
pub const COMMENT_LINE_START: &str = "% ";

/// Check if a line needs wrapping
#[must_use]
pub fn needs_wrap(
    line: &str,
    indent_length: usize,
    args: &Args,
    state: &State,
) -> bool {
    args.wrap
        && (line.chars().count() + indent_length > args.wraplen)
        && !(state.table.visual && args.format_tables)
}

fn is_wrap_point(
    i_byte: usize,
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
        && (i_byte + 1 < line_len)
}

/// Find the byte index of the closing delimiter of a verb command
/// starting at byte index `start`, such as
/// `\verb|...|`, `\lstinline[opts]{...}` or `\mintinline{lang}|...|`.
/// If no closing delimiter is found, the verb is assumed to extend
/// to the end of the line.
fn get_verb_end(start: usize, line: &str) -> usize {
    let last = line.len().saturating_sub(1);
    // Skip the command name
    let name_len = line[start + 1..]
        .find(|c: char| !c.is_ascii_alphabetic())
        .unwrap_or(line.len() - start - 1);
    let name = &line[start + 1..start + 1 + name_len];
    let mut pos = start + 1 + name_len;
    // Skip an optional star or [options] argument
    if line[pos..].starts_with('*') {
        pos += 1;
    }
    if line[pos..].starts_with('[') {
        match line[pos..].find(']') {
            Some(i) => pos += i + 1,
            None => return last,
        }
    }
    // Skip the {language} argument of \mintinline
    if name == "mintinline" && line[pos..].starts_with('{') {
        match line[pos..].find('}') {
            Some(i) => pos += i + 1,
            None => return last,
        }
    }
    // Find the matching closing delimiter
    let Some(open) = line[pos..].chars().next() else {
        return last;
    };
    let close = if open == '{' { '}' } else { open };
    let body = pos + open.len_utf8();
    line[body..].find(close).map_or(last, |i| body + i)
}

/// Find the byte ranges of all verb commands in a line
fn get_verb_ranges(line: &str) -> Vec<(usize, usize)> {
    let mut starts: Vec<usize> = VERBS
        .iter()
        .flat_map(|&x| line.match_indices(x).map(|(i, _)| i))
        .collect();
    starts.sort_unstable();
    let mut ranges: Vec<(usize, usize)> = vec![];
    for start in starts {
        if ranges.last().is_some_and(|&(_, end)| start <= end) {
            continue;
        }
        ranges.push((start, get_verb_end(start, line)));
    }
    ranges
}

fn is_inside_verb(i_byte: usize, verb_ranges: &[(usize, usize)]) -> bool {
    verb_ranges
        .iter()
        .any(|&(start, end)| start <= i_byte && i_byte <= end)
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
    let contains_verb =
        pattern.contains_verb && VERBS.iter().any(|x| line.contains(x));
    let verb_ranges = if contains_verb {
        get_verb_ranges(line)
    } else {
        vec![]
    };
    let mut after_non_percent = verb_ranges.first().map(|r| r.0) == Some(0);
    let wrap_boundary = args.wrapmin - indent_length;
    let line_len = line.len();

    for (i_char, (i_byte, c)) in line.char_indices().enumerate() {
        if i_char >= wrap_boundary && wrap_point.is_some() {
            break;
        }
        // Special wrapping for lines containing \verb|...|
        let inside_verb = is_inside_verb(i_byte, &verb_ranges);
        if is_wrap_point(i_byte, c, prev_c, inside_verb, line_len, args) {
            if after_non_percent {
                // Get index of the byte after which
                // line break will be inserted.
                // Note this may not be a valid char index.
                let wrap_byte = i_byte + c.len_utf8() - 1;
                // Don't wrap here if this is the end of the line anyway
                if wrap_byte + 1 < line_len {
                    wrap_point = Some(wrap_byte);
                }
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
    file: &Path,
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
        Some(p) if p <= args.wraplen => {}
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
