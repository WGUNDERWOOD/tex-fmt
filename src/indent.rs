use crate::comments::*;
use crate::format::*;
use crate::ignore::*;
use crate::leave::*;
use crate::logging::*;
use crate::parse::*;
use crate::regexes::*;
use crate::TAB;
use core::cmp::max;
use log::Level::{Trace, Warn};

const OPENS: [char; 3] = ['(', '[', '{'];
const CLOSES: [char; 3] = [')', ']', '}'];

#[derive(Debug, Clone)]
pub struct Indent {
    pub actual: i8,
    pub visual: i8,
}

impl Indent {
    pub fn new() -> Self {
        Indent {
            actual: 0,
            visual: 0,
        }
    }
}

// calculate total indentation change due to current line
fn get_diff(line: &str) -> i8 {
    // list environments get double indents
    let mut diff: i8 = 0;

    // other environments get single indents
    if RE_ENV_BEGIN.is_match(line) {
        // documents get no global indentation
        if RE_DOCUMENT_BEGIN.is_match(line) {
            return 0;
        };
        diff += 1;
        for re_list_begin in RE_LISTS_BEGIN.iter() {
            if re_list_begin.is_match(line) {
                diff += 1
            };
        }
    } else if RE_ENV_END.is_match(line) {
        // documents get no global indentation
        if RE_DOCUMENT_END.is_match(line) {
            return 0;
        };
        diff -= 1;
        for re_list_end in RE_LISTS_END.iter() {
            if re_list_end.is_match(line) {
                diff -= 1
            };
        }
    };

    // indent for delimiters
    diff += line.chars().filter(|x| OPENS.contains(x)).count() as i8;
    diff -= line.chars().filter(|x| CLOSES.contains(x)).count() as i8;

    diff
}

// calculate dedentation for current line compared to previous
fn get_back(line: &str) -> i8 {
    let mut back: i8 = 0;
    let mut cumul: i8 = 0;

    // delimiters
    for c in line.chars() {
        cumul -= OPENS.contains(&c) as i8;
        cumul += CLOSES.contains(&c) as i8;
        back = max(cumul, back);
    }

    // other environments get single indents
    if RE_ENV_END.is_match(line) {
        // documents get no global indentation
        if RE_DOCUMENT_END.is_match(line) {
            return 0;
        };
        // list environments get double indents for indenting items
        for re_list_end in RE_LISTS_END.iter() {
            if re_list_end.is_match(line) {
                return 2;
            };
        }
        back += 1;
    };

    // deindent items to make the rest of item environment appear indented
    if RE_ITEM.is_match(line) {
        back += 1;
    };

    back
}

fn get_indent(line: &str, prev_indent: &Indent) -> Indent {
    let diff = get_diff(line);
    let back = get_back(line);
    let actual = prev_indent.actual + diff;
    let visual = prev_indent.actual - back;
    Indent { actual, visual }
}

pub fn apply_indent(
    line: &str,
    state: &State,
    logs: &mut Vec<Log>,
    linum_new: usize,
    linum_old: usize,
    file: &str,
    args: &Cli,
) -> (String, State) {
    let mut new_line = line.to_string();
    let mut new_state = state.clone();

    new_state.ignore =
        get_ignore(line, state, logs, file, linum_new, linum_old, true);
    new_state.leave = get_leave(line, state, logs, file, true);

    if !new_state.leave.visual && !new_state.ignore.visual {
        // calculate indent
        let comment_index = find_comment_index(line);
        let line_strip = &remove_comment(line, comment_index);
        let mut indent = get_indent(line_strip, &state.indent);
        new_state.indent = indent.clone();
        let linum_new = 0; // TODO implement this
        let linum_old = 0; // TODO implement this
        if args.trace {
            record_line_log(
                logs,
                Trace,
                file,
                linum_new,
                linum_old,
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
                linum_new,
                linum_old,
                line,
                "Indent is negative.",
            );
            indent.actual = indent.actual.max(0);
            indent.visual = indent.visual.max(0);
        }

        // apply indent
        new_line = line.trim_start().to_string();
        if !new_line.is_empty() {
            let n_spaces = indent.visual * TAB;
            for _ in 0..n_spaces {
                new_line.insert(0, ' ');
            }
        }
    }

    (new_line, new_state)
}
