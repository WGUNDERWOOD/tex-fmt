use crate::ignore::*;
use crate::indent::*;
use crate::leave::*;
use crate::logging::*;
use crate::parse::*;
use crate::subs::*;
use crate::wrap::*;
use log::Level::{Info, Warn};

pub fn format_file(
    text: &str,
    file: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> String {
    record_file_log(logs, Info, file, "Formatting started.");
    let mut old_text = remove_extra_newlines(text);
    old_text = environments_new_line(&old_text, file, args, logs);
    old_text = remove_tabs(&old_text);
    old_text = remove_trailing_spaces(&old_text);

    let mut state = State::new();
    let mut old_lines: Vec<&str> = old_text.lines().rev().collect();
    let mut queue: Vec<String> = vec![];
    let mut new_text: String = "".to_string();

    loop {
        if !queue.is_empty() {
            // process the queue
            let mut line = queue.pop().unwrap();
            let temp_state: State;
            let linum_new = 0; // TODO implement this
            let linum_old = 0; // TODO implement this
            (line, temp_state) = apply_indent(
                &line, &state, logs, linum_new, linum_old, file, args,
            );
            if needs_wrap(&line, &state, file, logs) {
                let wrapped_lines =
                    apply_wrap(&line, linum_new, linum_old, file, args, logs);
                if wrapped_lines.is_some() {
                    queue.push(wrapped_lines.clone().unwrap().1);
                    queue.push(wrapped_lines.clone().unwrap().0);
                } else {
                    new_text.push_str(&line);
                    new_text.push('\n');
                };
            } else {
                state = temp_state;
                state.linum_new += 1;
                new_text.push_str(&line);
                new_text.push('\n');
            }
        } else if !old_lines.is_empty() {
            // move the next line into the queue
            let line: String = old_lines.pop().unwrap().to_string();
            queue.push(line);
        } else {
            break;
        }
    }

    new_text = remove_trailing_spaces(&new_text);

    if !indents_return_to_zero(&new_text) {
        record_file_log(logs, Warn, file, "Indent does not return to zero.");
    }

    record_file_log(logs, Info, file, "Formatting complete.");

    new_text
}

#[derive(Clone)]
pub struct State {
    pub linum_old: usize,
    pub linum_new: usize,
    pub ignore: Ignore,
    pub indent: Indent,
    pub leave: Leave,
}

impl State {
    pub fn new() -> Self {
        State {
            linum_old: 0,
            linum_new: 0,
            ignore: Ignore::new(),
            indent: Indent::new(),
            leave: Leave::new(),
        }
    }
}

fn indents_return_to_zero(text: &str) -> bool {
    !text.lines().last().unwrap_or_default().starts_with(' ')
}
