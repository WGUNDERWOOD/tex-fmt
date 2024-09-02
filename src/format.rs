use crate::ignore::*;
use crate::indent::*;
use crate::leave::*;
use crate::logging::*;
use crate::parse::*;
use crate::subs::*;
use crate::wrap::*;
use crate::LINE_END;
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

    let mut old_lines = old_text.lines().enumerate();

    let mut queue: Vec<(usize, String)> = vec![];
    let mut new_text = String::with_capacity(text.len());

    loop {
        if let Some((linum_old, mut line)) = queue.pop() {
            let temp_state: State;
            (line, temp_state) =
                apply_indent(&line, linum_old, &state, logs, file, args);
            if needs_wrap(&line, &temp_state, args) {
                let wrapped_lines =
                    apply_wrap(&line, &temp_state, file, args, logs);
                if wrapped_lines.is_some() {
                    queue.push((linum_old, wrapped_lines.clone().unwrap().1));
                    queue.push((linum_old, wrapped_lines.clone().unwrap().0));
                } else {
                    new_text.push_str(&line);
                    new_text.push_str(LINE_END);
                    state.linum_new += 1;
                };
            } else {
                state = temp_state;
                new_text.push_str(&line);
                new_text.push_str(LINE_END);
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

    record_file_log(logs, Info, file, "Formatting complete.");

    new_text
}

#[derive(Clone, Debug)]
pub struct State {
    pub linum_old: usize,
    pub linum_new: usize,
    pub ignore: Ignore,
    pub indent: Indent,
    pub leave: Leave,
}

impl State {
    pub const fn new() -> Self {
        Self {
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
