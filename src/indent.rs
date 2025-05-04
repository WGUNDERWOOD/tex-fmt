//! Utilities for indenting source lines

use crate::args::Args;
use crate::comments::{find_comment_index, remove_comment};
use crate::format::{Pattern, State};
use crate::logging::{record_line_log, Log};
use crate::regexes::{ENV_BEGIN, ENV_END, ITEM, VERB};
use core::cmp::max;
use log::Level;
use log::LevelFilter;

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
    if pattern.contains_verb && line.contains(VERB) {
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

    // Indentation for delimiters
    diff += line
        .chars()
        .map(|x| i8::from(OPENS.contains(&x)) - i8::from(CLOSES.contains(&x)))
        .sum::<i8>();

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
    if pattern.contains_verb && line.contains(VERB) {
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

    // Dedent delimiters
    let mut cumul: i8 = back;
    for c in line.chars() {
        cumul -= i8::from(OPENS.contains(&c));
        cumul += i8::from(CLOSES.contains(&c));
        back = max(cumul, back);
    }

    back
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
    let diff = get_diff(
        line,
        pattern,
        lists_begin,
        lists_end,
        no_indent_envs_begin,
        no_indent_envs_end,
    );
    let back = get_back(line, pattern, state, lists_end, no_indent_envs_end);
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
    file: &str,
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
