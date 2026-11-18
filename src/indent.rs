//! Utilities for indenting source lines

use crate::args::Args;
use crate::comments::{find_comment_index, remove_comment};
use crate::format::{Pattern, State};
use crate::logging::{record_line_log, Log};
use crate::regexes::{ENV_BEGIN, ENV_END, ITEM, VERBS};
use core::cmp::max;
use log::Level;
use log::LevelFilter;
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;

/// Opening delimiters
const OPENS: [char; 3] = ['{', '(', '['];
/// Closing delimiters
const CLOSES: [char; 3] = ['}', ')', ']'];

/// Information on the indentation state of a line
#[derive(Debug, Clone)]
pub struct Indent {
    /// The indentation level of a line
    pub actual: i8,
    /// The visual indentation level of a line
    pub visual: i8,
}

impl Indent {
    /// Construct a new indentation state
    #[must_use]
    pub const fn new() -> Self {
        Self {
            actual: 0,
            visual: 0,
        }
    }
}

impl Default for Indent {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate total indentation change due to the current line
fn get_diff(
    line: &str,
    pattern: &Pattern,
    lists_begin: &[String],
    lists_end: &[String],
    no_indent_envs_begin: &[String],
    no_indent_envs_end: &[String],
) -> i8 {
    // Do not indent if line contains \verb|...|
    if pattern.contains_verb && VERBS.iter().any(|x| line.contains(x)) {
        return 0;
    }

    // Indentation for environments
    let mut diff: i8 = 0;
    if pattern.contains_env_begin && line.contains(ENV_BEGIN) {
        if no_indent_envs_begin.iter().any(|r| line.contains(r)) {
            return 0;
        }
        diff += 1;
        diff += i8::from(lists_begin.iter().any(|r| line.contains(r)));
    } else if pattern.contains_env_end && line.contains(ENV_END) {
        if no_indent_envs_end.iter().any(|r| line.contains(r)) {
            return 0;
        }
        diff -= 1;
        diff -= i8::from(lists_end.iter().any(|r| line.contains(r)));
    }

    diff
}

/// Calculate dedentation for the current line
fn get_back(
    line: &str,
    pattern: &Pattern,
    state: &State,
    lists_end: &[String],
    no_indent_envs_end: &[String],
) -> i8 {
    // Only need to dedent if indentation is present
    if state.indent.actual == 0 {
        return 0;
    }
    let mut back: i8 = 0;

    // Don't apply any indenting if a \verb|...| is present
    if pattern.contains_verb && VERBS.iter().any(|x| line.contains(x)) {
        return 0;
    }

    // Calculate dedentation for environments
    if pattern.contains_env_end && line.contains(ENV_END) {
        // Some environments are not indented
        if no_indent_envs_end.iter().any(|r| line.contains(r)) {
            return 0;
        }
        // List environments get double indents for indenting items
        for r in lists_end {
            if line.contains(r) {
                return 2;
            }
        }
        // Other environments get single indents
        back = 1;
    } else if pattern.contains_item && line.contains(ITEM) {
        // Deindent items to make the rest of item environment appear indented
        back += 1;
    }

    back
}

/// Count the delimiters on a line, in order
///
/// `\left` and `\right` count as an opening and a closing delimiter,
/// whichever delimiter follows them, and that delimiter is not counted
/// again. So `\left\{ ... \right.` and `\left[ ... \right)` are balanced,
/// as in LaTeX, where `\left` must be matched by `\right`.
fn get_diff_back_delim(line: &str) -> (i8, i8) {
    let mut diff: i8 = 0;
    let mut back: i8 = 0;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match read_left_right(&mut chars) {
                Some(LeftRight::Left) => diff -= 1,
                Some(LeftRight::Right) => diff += 1,
                None => {}
            }
        } else {
            diff -= i8::from(OPENS.contains(&c));
            diff += i8::from(CLOSES.contains(&c));
        }
        back = max(diff, back);
    }
    (-diff, back)
}

/// A `\left` or a `\right`
enum LeftRight {
    Left,
    Right,
}

/// Read the command after a backslash; for `\left` and `\right`, also skip
/// the delimiter which follows
///
/// An escaped backslash `\\` is skipped. Other escaped characters, such as
/// `\{`, are left to be counted as before.
fn read_left_right(chars: &mut Peekable<Chars>) -> Option<LeftRight> {
    if chars.next_if_eq(&'\\').is_some() {
        return None;
    }
    let mut name = String::new();
    while let Some(c) = chars.next_if(char::is_ascii_alphabetic) {
        name.push(c);
    }
    let command = match name.as_str() {
        "left" => LeftRight::Left,
        "right" => LeftRight::Right,
        _ => return None,
    };
    skip_delimiter(chars);
    Some(command)
}

/// Skip the delimiter after `\left` or `\right`: a control word such as
/// `\langle`, a control symbol such as `\{` or `\|`, or a character such
/// as `(` or `.`
fn skip_delimiter(chars: &mut Peekable<Chars>) {
    while chars.next_if(|c| c.is_whitespace()).is_some() {}
    if chars.next() == Some('\\') {
        if chars.next_if(char::is_ascii_alphabetic).is_some() {
            while chars.next_if(char::is_ascii_alphabetic).is_some() {}
        } else {
            chars.next();
        }
    }
}

/// Calculate indentation properties of the current line
#[allow(clippy::too_many_arguments)]
fn get_indent(
    line: &str,
    prev_indent: &Indent,
    pattern: &Pattern,
    state: &State,
    lists_begin: &[String],
    lists_end: &[String],
    no_indent_envs_begin: &[String],
    no_indent_envs_end: &[String],
) -> Indent {
    let mut diff = get_diff(
        line,
        pattern,
        lists_begin,
        lists_end,
        no_indent_envs_begin,
        no_indent_envs_end,
    );
    let mut back =
        get_back(line, pattern, state, lists_end, no_indent_envs_end);
    let diff_back_delim = get_diff_back_delim(line);
    diff += diff_back_delim.0;
    back += diff_back_delim.1;
    let actual = prev_indent.actual + diff;
    let visual = prev_indent.actual - back;
    Indent { actual, visual }
}

/// Calculates the indent for `line` based on its contents.
/// This functions saves the calculated [Indent], which might be
/// negative, to the given [State], and then ensures that the returned
/// [Indent] is non-negative.
#[allow(clippy::too_many_arguments)]
pub fn calculate_indent(
    line: &str,
    state: &mut State,
    logs: &mut Vec<Log>,
    file: &Path,
    args: &Args,
    pattern: &Pattern,
    lists_begin: &[String],
    lists_end: &[String],
    no_indent_envs_begin: &[String],
    no_indent_envs_end: &[String],
) -> Indent {
    // Calculate the new indent by first removing the comment from the line
    // (if there is one) to ignore diffs from characters in there.
    let comment_index = find_comment_index(line, pattern);
    let line_strip = remove_comment(line, comment_index);
    let mut indent = get_indent(
        line_strip,
        &state.indent,
        pattern,
        state,
        lists_begin,
        lists_end,
        no_indent_envs_begin,
        no_indent_envs_end,
    );

    // Record the indent to the logs.
    if args.verbosity == LevelFilter::Trace {
        record_line_log(
            logs,
            Level::Trace,
            file,
            state.linum_new,
            state.linum_old,
            line,
            &format!(
                "Indent: actual = {}, visual = {}:",
                indent.actual, indent.visual
            ),
        );
    }

    // Save the indent to the state. Note, this indent might be negative;
    // it is saved without correction so that this is
    // not forgotten for the next iterations.
    state.indent = indent.clone();

    // Update the last zero-indented line for use in error messages.
    if indent.visual == 0 && state.linum_new > state.linum_last_zero_indent {
        state.linum_last_zero_indent = state.linum_new;
    }

    // However, we can't negatively indent a line.
    // So we log the negative indent and reset the values to 0.
    if (indent.visual < 0) || (indent.actual < 0) {
        record_line_log(
            logs,
            Level::Warn,
            file,
            state.linum_new,
            state.linum_old,
            line,
            "Indent is negative.",
        );
        indent.actual = indent.actual.max(0);
        indent.visual = indent.visual.max(0);

        // If this is the first negatively indented line, record in the state
        if state.linum_first_negative_indent.is_none() {
            state.linum_first_negative_indent = Some(state.linum_new);
        }
    }

    indent
}

/// Apply the given indentation to a line
#[must_use]
pub fn apply_indent(
    line: &str,
    indent: &Indent,
    args: &Args,
    indent_char: &str,
) -> String {
    let first_non_whitespace = line.chars().position(|c| !c.is_whitespace());

    // If line is blank, return an empty line
    if first_non_whitespace.is_none() {
        return String::new();
    }

    // If line is correctly indented, return it directly
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    let n_indent_chars = (indent.visual * args.tabsize as i8) as usize;
    if first_non_whitespace == Some(n_indent_chars) {
        return line.into();
    }

    // Otherwise, allocate enough memory to fit line with the added
    // indentation and insert the appropriate string slices
    let trimmed_line = line.trim_start();
    let mut new_line =
        String::with_capacity(trimmed_line.len() + n_indent_chars);
    for idx in 0..n_indent_chars {
        new_line.insert_str(idx, indent_char);
    }
    new_line.insert_str(n_indent_chars, trimmed_line);
    new_line
}
