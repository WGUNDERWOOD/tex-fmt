use crate::ignore::*;
use crate::indent::*;
use crate::leave::*;
use crate::logging::*;
use crate::parse::*;
use crate::subs::*;
use crate::wrap::*;
use log::Level::{Info, Warn};
use std::iter::zip;

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

    let old_lines = old_text.lines().rev();
    let linums_old = (1..old_lines.clone().count() + 1).rev();
    let mut old_lines: Vec<(usize, &str)> =
        zip(linums_old, old_lines).collect();

    let mut queue: Vec<(usize, String)> = vec![];
    let mut new_text: String = "".to_string();

    loop {
        if let Some((linum_old, mut line)) = queue.pop() {
            //println!("{}", &line);
            //dbg!(linum_old);
            //println!("\n");
            let temp_state: State;
            (line, temp_state) =
                apply_indent(&line, linum_old, &state, logs, file, args);
            if needs_wrap(&line, &temp_state) {
                let wrapped_lines =
                    apply_wrap(&line, &temp_state, file, args, logs);
                if wrapped_lines.is_some() {
                    queue.push((linum_old, wrapped_lines.clone().unwrap().1));
                    queue.push((linum_old, wrapped_lines.clone().unwrap().0));
                } else {
                    new_text.push_str(&line);
                    //dbg!(&state);
                    new_text.push('\n');
                };
            } else {
                state = temp_state;
                new_text.push_str(&line);
                //println!("{}", &line);
                //dbg!(&state.linum_old);
                //dbg!(&state.linum_new);
                //println!("\n");
                new_text.push('\n');
            }
        } else if let Some((linum_old, line)) = old_lines.pop() {
            queue.push((linum_old, line.to_string()));
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

#[derive(Clone, Debug)]
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
