//! Utilities for indenting source lines

use crate::args::*;
use crate::comments::*;
use crate::format::*;
use crate::logging::*;
use crate::pattern::Pattern;
use crate::regexes::*;
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
    pub const fn new() -> Self {
        Self {
            actual: 0,
            visual: 0,
        }
    }
}

/// Calculate total indentation change due to the current line
fn get_diff(line: &str, pattern: &Pattern) -> i8 {
    // list environments get double indents
    let mut diff: i8 = 0;

    // other environments get single indents
    if pattern.contains_env_begin && line.contains(ENV_BEGIN) {
        // documents get no global indentation
        if line.contains(DOC_BEGIN) {
            return 0;
        };
        diff += 1;
        diff += i8::from(LISTS_BEGIN.iter().any(|r| line.contains(r)));
    } else if pattern.contains_env_end && line.contains(ENV_END) {
        // documents get no global indentation
        if line.contains(DOC_END) {
            return 0;
        };
        diff -= 1;
        diff -= i8::from(LISTS_END.iter().any(|r| line.contains(r)));
    };

    // indent for delimiters
    diff += line
        .chars()
        .map(|x| i8::from(OPENS.contains(&x)) - i8::from(CLOSES.contains(&x)))
        .sum::<i8>();

    diff
}

/// Calculate dedentation for the current line
fn get_back(line: &str, pattern: &Pattern, state: &State) -> i8 {
    // Only need to dedent if indentation is present
    if state.indent.actual == 0 {
        return 0;
    }
    let mut back: i8 = 0;

    if pattern.contains_env_end && line.contains(ENV_END) {
        // documents get no global indentation
        if line.contains(DOC_END) {
            return 0;
        };
        // list environments get double indents for indenting items
        for r in LISTS_END.iter() {
            if line.contains(r) {
                return 2;
            };
        }
        // other environments get single indents
        back = 1;
    } else if pattern.contains_item && line.contains(ITEM) {
        // deindent items to make the rest of item environment appear indented
        back += 1;
    };

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
fn get_indent(
    line: &str,
    prev_indent: &Indent,
    pattern: &Pattern,
    state: &State,
) -> Indent {
    let diff = get_diff(line, pattern);
    let back = get_back(line, pattern, state);
    let actual = prev_indent.actual + diff;
    let visual = prev_indent.actual - back;
    Indent { actual, visual }
}

/// Calculates the indent for `line` based on its contents.
/// This functions saves the calculated [Indent], which might be
/// negative, to the given [State], and then ensures that the returned
/// [Indent] is non-negative.
pub fn calculate_indent(
    line: &str,
    state: &mut State,
    logs: &mut Vec<Log>,
    file: &str,
    args: &Args,
    pattern: &Pattern,
) -> Indent {
    // Calculate the new indent by first removing the comment from the line
    // (if there is one) to ignore diffs from characters in there.
    let comment_index = find_comment_index(line);
    let line_strip = remove_comment(line, comment_index);
    let mut indent = get_indent(line_strip, &state.indent, pattern, state);

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
