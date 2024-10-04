//! Utilities for indenting source lines

use crate::comments::*;
use crate::format::*;
use crate::ignore::*;
use crate::logging::*;
use crate::parse::*;
use crate::regexes::*;
use crate::verbatim::*;
use core::cmp::max;
use log::Level::{Trace, Warn};

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
        // double check here as pattern might have matched inside comment
        // documents get no global indentation
        if line.contains(DOC_BEGIN) {
            return 0;
        };
        diff += 1;
        diff += i8::try_from(
            LISTS_BEGIN.iter().filter(|&r| line.contains(r)).count(),
        )
        .unwrap();
    } else if pattern.contains_env_end && line.contains(ENV_END) {
        // double check here as pattern might have matched inside comment
        // documents get no global indentation
        if line.contains(DOC_END) {
            return 0;
        };
        diff -= 1;
        diff -= i8::try_from(
            LISTS_END.iter().filter(|&r| line.contains(r)).count(),
        )
        .unwrap();
    };

    // indent for delimiters
    diff += i8::try_from(line.chars().filter(|x| OPENS.contains(x)).count())
        .unwrap();
    diff -= i8::try_from(line.chars().filter(|x| CLOSES.contains(x)).count())
        .unwrap();

    diff
}

/// Calculate dedentation for the current line
fn get_back(line: &str, pattern: &Pattern) -> i8 {
    let mut back: i8 = 0;
    let mut cumul: i8 = 0;

    // delimiters
    for c in line.chars() {
        cumul -= i8::from(OPENS.contains(&c));
        cumul += i8::from(CLOSES.contains(&c));
        back = max(cumul, back);
    }

    // other environments get single indents
    if pattern.contains_env_end {
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
        back += 1;
    };

    // deindent items to make the rest of item environment appear indented
    if pattern.contains_item && line.contains(ITEM) {
        back += 1;
    };

    back
}

/// Calculate indentation properties of the current line
fn get_indent(line: &str, prev_indent: &Indent, pattern: &Pattern) -> Indent {
    let diff = get_diff(line, pattern);
    let back = get_back(line, pattern);
    let actual = prev_indent.actual + diff;
    let visual = prev_indent.actual - back;
    Indent { actual, visual }
}

/// Apply the correct indentation to a line
pub fn apply_indent(
    line: &str,
    linum_old: usize,
    state: &State,
    logs: &mut Vec<Log>,
    file: &str,
    args: &Cli,
    pattern: &Pattern,
) -> (String, State) {
    let mut new_line = line.to_string();
    let mut new_state = state.clone();
    new_state.linum_old = linum_old;

    new_state.ignore = get_ignore(line, &new_state, logs, file, true);
    new_state.verbatim =
        get_verbatim(line, &new_state, logs, file, true, pattern);

    if !new_state.verbatim.visual && !new_state.ignore.visual {
        // calculate indent
        let comment_index = find_comment_index(line);
        let line_strip = &remove_comment(line, comment_index);
        let mut indent = get_indent(line_strip, &state.indent, pattern);
        new_state.indent = indent.clone();
        if args.trace {
            record_line_log(
                logs,
                Trace,
                file,
                state.linum_new,
                new_state.linum_old,
                line,
                &format!(
                    "Indent: actual = {}, visual = {}:",
                    indent.actual, indent.visual
                ),
            );
        }

        if (indent.visual < 0) || (indent.actual < 0) {
            record_line_log(
                logs,
                Warn,
                file,
                new_state.linum_new,
                new_state.linum_old,
                line,
                "Indent is negative.",
            );
            indent.actual = indent.actual.max(0);
            indent.visual = indent.visual.max(0);
        }

        // apply indent
        trim_start_in_place(&mut new_line);
        if !new_line.is_empty() {
            let n_indent_chars = indent.visual * args.tab;
            for _ in 0..n_indent_chars {
                if args.usetabs {
                    new_line.insert(0, '\t');
                } else {
                    new_line.insert(0, ' ');
                }
            }
        }
    }

    (new_line, new_state)
}

/// Faster, in-place implementation of trim_start
fn trim_start_in_place(s: &mut String) {
    let trimmed: &str = s.trim_start();
    let start = trimmed.as_ptr() as usize - s.as_ptr() as usize;
    let len = trimmed.len();
    if start != 0 {
        s.drain(..start);
    }
    s.truncate(len);
}
