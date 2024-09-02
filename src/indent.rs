use crate::comments::*;
use crate::format::*;
use crate::ignore::*;
use crate::logging::*;
use crate::parse::*;
use crate::regexes::*;
use crate::verbatim::*;
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
    pub const fn new() -> Self {
        Self {
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
    if line.contains(ENV_BEGIN) {
        // documents get no global indentation
        if line.contains(DOC_BEGIN) {
            return 0;
        };
        diff += 1;
        diff += i8::try_from(
            LISTS_BEGIN.iter().filter(|&r| line.contains(r)).count(),
        )
        .unwrap();
    } else if line.contains(ENV_END) {
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

// calculate dedentation for current line compared to previous
fn get_back(line: &str) -> i8 {
    let mut back: i8 = 0;
    let mut cumul: i8 = 0;

    // delimiters
    for c in line.chars() {
        cumul -= i8::from(OPENS.contains(&c));
        cumul += i8::from(CLOSES.contains(&c));
        back = max(cumul, back);
    }

    // other environments get single indents
    if line.contains(ENV_END) {
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
    if line.contains(ITEM) {
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
    linum_old: usize,
    state: &State,
    logs: &mut Vec<Log>,
    file: &str,
    args: &Cli,
) -> (String, State) {
    let mut new_line = line.to_string();
    let mut new_state = state.clone();
    new_state.linum_new += 1;
    new_state.linum_old = linum_old;

    new_state.ignore = get_ignore(line, &new_state, logs, file, true);
    new_state.verbatim = get_verbatim(line, &new_state, logs, file, true);

    if !new_state.verbatim.visual && !new_state.ignore.visual {
        // calculate indent
        let comment_index = find_comment_index(line);
        let line_strip = &remove_comment(line, comment_index);
        let mut indent = get_indent(line_strip, &state.indent);
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
