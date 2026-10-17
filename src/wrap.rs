//! Utilities for wrapping long lines

use crate::args::Args;
use crate::comments::find_comment_index;
use crate::format::{Pattern, State};
use crate::indent::get_diff_back_delim;
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

fn get_verb_end(verb_byte_start: Option<usize>, line: &str) -> Option<usize> {
    verb_byte_start.map(|v| {
        line[v..]
            .match_indices(['|', '+'])
            .nth(1)
            .map(|(i, _)| i + v)
    })?
}

fn is_inside_verb(
    i_byte: usize,
    contains_verb: bool,
    verb_start: Option<usize>,
    verb_end: Option<usize>,
) -> bool {
    if !contains_verb {
        return false;
    }
    // For some verb-like commands (e.g. `\mintinline{...}{...}`) we cannot
    // locate a `|` / `+` delimiter, so `verb_end` may be `None`. Treat that
    // as "no protected region" rather than panicking.
    match (verb_start, verb_end) {
        (Some(start), Some(end)) => start <= i_byte && i_byte <= end,
        _ => false,
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
    let contains_verb =
        pattern.contains_verb && VERBS.iter().any(|x| line.contains(x));
    let verb_start: Option<usize> = contains_verb
        .then(|| VERBS.iter().filter_map(|&x| line.find(x)).min().unwrap());

    let verb_end = get_verb_end(verb_start, line);
    let mut after_non_percent = verb_start == Some(0);
    let wrap_boundary = args.wrapmin - indent_length;
    let line_len = line.len();

    for (i_char, (i_byte, c)) in line.char_indices().enumerate() {
        if i_char >= wrap_boundary && wrap_point.is_some() {
            break;
        }
        // Special wrapping for lines containing \verb|...|
        let inside_verb =
            is_inside_verb(i_byte, contains_verb, verb_start, verb_end);
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

/// Check if a line contains a character not preceded by a backslash
fn contains_unescaped(line: &str, target: char) -> bool {
    let mut prev_c: Option<char> = None;
    for c in line.chars() {
        if c == target && prev_c != Some('\\') {
            return true;
        }
        prev_c = Some(c);
    }
    false
}

/// Check if a line ends with a LaTeX line break: `\\`, `\\*`,
/// `\\[length]` or `\\*[length]`.
fn ends_with_line_break(line: &str) -> bool {
    let mut line = line.trim_end();
    if line.ends_with(']') {
        match line.rfind('[') {
            Some(i) => line = &line[..i],
            None => return false,
        }
    }
    line.strip_suffix('*').unwrap_or(line).ends_with("\\\\")
}

/// Check if a line is a complete structural command, such as a
/// `\begin{...}` or `\section{...}` with balanced brackets, which
/// cannot have been wrapped onto the following line.
fn is_complete_structural(line: &str, pattern: &Pattern) -> bool {
    pattern.contains_splitting
        && !pattern.contains_item
        && get_diff_back_delim(line).0 <= 0
}

/// Check if a line is the continuation of a wrapped previous line.
///
/// A line counts as a continuation when appending its first unbreakable
/// chunk to the previous line would overflow the wrap boundary, which is
/// exactly the condition under which [`find_wrap_point`] breaks a line
/// there. Because this holds both for freshly wrapped lines and for
/// already-wrapped lines read from disk, the extra indentation given to
/// continuations survives reformatting.
///
/// Lines which conventionally start fresh are never continuations:
/// structural lines containing splitting patterns, comment lines,
/// alignment rows containing an unescaped `&`, `\intertext` lines, and
/// lines following a line break `\\`, a comment, or a complete
/// structural command.
#[must_use]
pub fn is_wrap_continuation(
    line: &str,
    prev_line: &str,
    prev_pattern: &Pattern,
    args: &Args,
    pattern: &Pattern,
) -> bool {
    if !args.wrap_indent || !args.wrap || pattern.contains_splitting {
        return false;
    }

    // The previous line cannot have been wrapped onto this one.
    let prev = prev_line.trim_end();
    if prev.is_empty()
        || ends_with_line_break(prev)
        || contains_unescaped(prev, '%')
        || is_complete_structural(prev, prev_pattern)
    {
        return false;
    }

    // This line starts fresh by convention.
    let trimmed = line.trim_start();
    if trimmed.is_empty()
        || trimmed.starts_with('%')
        || trimmed.starts_with("\\intertext{")
        || trimmed.starts_with("\\shortintertext{")
        || contains_unescaped(trimmed, '&')
    {
        return false;
    }

    // Measure the first unbreakable chunk of this line.
    let mut chunk_len: usize = 0;
    let mut prev_c: Option<char> = None;
    for c in trimmed.chars() {
        if args.wrap_chars.contains(&c) && prev_c != Some('\\') {
            break;
        }
        chunk_len += 1;
        prev_c = Some(c);
    }

    // Joining this chunk onto the previous line would overflow. The break
    // consumed a space that trailing-space removal strips from the previous
    // line, so the boundary case is >= rather than >.
    prev.chars().count() + 1 + chunk_len >= args.wrapmin
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
