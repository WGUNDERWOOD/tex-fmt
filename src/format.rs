//! Core methodology for formatting a file

use crate::ignore::*;
use crate::indent::*;
use crate::logging::*;
use crate::parse::*;
use crate::subs::*;
use crate::verbatim::*;
use crate::wrap::*;
use crate::LINE_END;
use log::Level::{Info, Warn};
use std::iter::zip;
use texlab::parser::parse_latex;
use texlab::syntax::latex::SyntaxNode;

pub fn format_file(
    text: &str,
    file: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> String {
    record_file_log(logs, Info, file, "Formatting started.");
    let mut old_text = remove_extra_newlines(text);
    old_text = remove_tabs(&old_text, args);
    old_text = remove_trailing_spaces(&old_text);

    let mut state = State::new();
    let old_lines = old_text.lines();
    let mut old_lines = zip(1.., old_lines);
    let mut queue: Vec<(usize, String)> = vec![];
    let mut new_text = String::with_capacity(text.len());

    loop {
        if let Some((linum_old, mut line)) = queue.pop() {
            let temp_state: State;
            (line, temp_state) =
                apply_indent(&line, linum_old, &state, logs, file, args);
            if needs_env_new_line(&line, &temp_state) {
                let env_lines =
                    put_env_new_line(&line, &temp_state, file, args, logs);
                if env_lines.is_some() {
                    queue.push((linum_old, env_lines.clone().unwrap().1));
                    queue.push((linum_old, env_lines.clone().unwrap().0));
                } else {
                    state = temp_state;
                    new_text.push_str(&line);
                    new_text.push_str(LINE_END);
                    state.linum_new += 1;
                };
            } else if needs_wrap(&line, &temp_state, args) {
                let wrapped_lines =
                    apply_wrap(&line, &temp_state, file, args, logs);
                if wrapped_lines.is_some() {
                    queue.push((linum_old, wrapped_lines.clone().unwrap().1));
                    queue.push((linum_old, wrapped_lines.clone().unwrap().0));
                } else {
                    state = temp_state;
                    new_text.push_str(&line);
                    new_text.push_str(LINE_END);
                    state.linum_new += 1;
                };
            } else {
                state = temp_state;
                new_text.push_str(&line);
                new_text.push_str(LINE_END);
                state.linum_new += 1;
            }
        } else if let Some((linum_old, line)) = old_lines.next() {
            queue.push((linum_old, line.to_string()));
        } else {
            break;
        }
    }

    if !indents_return_to_zero(&new_text) {
        record_file_log(logs, Warn, file, "Indent does not return to zero.");
    }

    new_text = remove_trailing_spaces(&new_text);
    record_file_log(logs, Info, file, "Formatting complete.");

    let tree = SyntaxNode::new_root(parse_latex(&text));
    dbg!(&tree);

    let new_text = tree.to_string();
    new_text
}

/// Information on the current state during formatting
#[derive(Clone, Debug)]
pub struct State {
    /// Corresponding line number in the original file
    pub linum_old: usize,
    /// Corresponding line number in the formatted file
    pub linum_new: usize,
    /// Ignored status of the current line
    pub ignore: Ignore,
    /// Indentation status of the current line
    pub indent: Indent,
    /// Verbatim status of the current line
    pub verbatim: Verbatim,
}

impl State {
    /// Construct a new default state
    pub const fn new() -> Self {
        Self {
            linum_old: 1,
            linum_new: 1,
            ignore: Ignore::new(),
            indent: Indent::new(),
            verbatim: Verbatim::new(),
        }
    }
}

/// Ensure that the indentation returns to zero at the end of the file
fn indents_return_to_zero(text: &str) -> bool {
    !text.lines().last().unwrap_or_default().starts_with(' ')
}
