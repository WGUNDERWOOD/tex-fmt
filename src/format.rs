//! Core methodology for formatting a file

use crate::cli::*;
use crate::ignore::*;
use crate::indent::*;
use crate::logging::*;
use crate::regexes::{ENV_BEGIN, ENV_END, ITEM};
use crate::subs::*;
use crate::verbatim::*;
use crate::wrap::*;
use crate::LINE_END;
use log::Level::{Info, Warn};
use std::iter::zip;

/// Central function to format a file
pub fn format_file(
    old_text: &str,
    file: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> String {
    record_file_log(logs, Info, file, "Formatting started.");

    // Clean the source file and zip its lines with line numbers
    let old_text = clean_text(old_text, args);
    let mut old_lines = zip(1.., old_text.lines());

    // Initialise
    let mut state = State::new();
    let mut queue: Vec<(usize, String)> = vec![];
    let mut new_text = String::with_capacity(2 * old_text.len());

    // Select the character used for indentation.
    let indent_char = if args.usetabs { "\t" } else { " " };

    loop {
        if let Some((linum_old, mut line)) = queue.pop() {
            // Read the patterns present on this line.
            let pattern = Pattern::new(&line);

            // Temporary state for working on this line.
            let mut temp_state = state.clone();

            // Update the state with the line number from the queue.
            temp_state.linum_old = linum_old;

            // If the line should not be ignored ...
            if !set_ignore_and_report(
                &line,
                &mut temp_state,
                logs,
                file,
                &pattern,
            ) {
                // Check if the line should be split because of a pattern that should begin on a new line.
                if needs_env_new_line(&line, &temp_state, &pattern) {
                    // Split the line into two ...
                    let (this_line, next_line) =
                        put_env_new_line(&line, &temp_state, file, args, logs);
                    // ... and queue the second part for formatting.
                    queue.push((linum_old, next_line.to_string()));
                    line = this_line.to_string();
                }

                // Calculate the indent based on the current state and the patterns in the line.
                let indent = calculate_indent(
                    &line,
                    &mut temp_state,
                    logs,
                    file,
                    args,
                    &pattern,
                );

                let indent_length = usize::try_from(indent.visual * args.tab)
                    .expect("Visual indent is non-negative.");

                // Wrap the line before applying the indent, and loop back if the line needed wrapping.
                if needs_wrap(line.trim_start(), indent_length, args) {
                    let wrapped_lines = apply_wrap(
                        line.trim_start(),
                        indent_length,
                        &temp_state,
                        file,
                        args,
                        logs,
                    );
                    if let Some([this_line, next_line_start, next_line]) =
                        wrapped_lines
                    {
                        queue.push((
                            linum_old,
                            [next_line_start, next_line].concat(),
                        ));
                        queue.push((linum_old, this_line.to_string()));
                        continue;
                    }
                }

                // Lastly, apply the indent if the line didn't need wrapping.
                line = apply_indent(&line, &indent, args, indent_char);
            }

            // Add line to new text
            state = temp_state;
            new_text.push_str(&line);
            new_text.push_str(LINE_END);
            state.linum_new += 1;
        } else if let Some((linum_old, line)) = old_lines.next() {
            queue.push((linum_old, line.to_string()));
        } else {
            break;
        }
    }

    if !indents_return_to_zero(&state) {
        record_file_log(logs, Warn, file, "Indent does not return to zero.");
    }

    new_text = remove_trailing_spaces(&new_text);
    record_file_log(logs, Info, file, "Formatting complete.");
    new_text
}

/// Sets the `ignore` and `verbatim` flags in the given [State] based on `line` and returns whether `line` should be
/// ignored by formatting.
fn set_ignore_and_report(
    line: &str,
    temp_state: &mut State,
    logs: &mut Vec<Log>,
    file: &str,
    pattern: &Pattern,
) -> bool {
    temp_state.ignore = get_ignore(line, temp_state, logs, file, true);
    temp_state.verbatim =
        get_verbatim(line, temp_state, logs, file, true, pattern);

    temp_state.verbatim.visual || temp_state.ignore.visual
}

/// Cleans the given text by removing extra line breaks and trailing spaces, and tabs if they shouldn't be used.
fn clean_text(text: &str, args: &Cli) -> String {
    let mut text = remove_extra_newlines(text);

    if !args.usetabs {
        text = remove_tabs(&text, args);
    }

    text = remove_trailing_spaces(&text);

    text
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

/// Record whether a line contains certain patterns to avoid recomputing
pub struct Pattern {
    /// Whether a begin environment pattern is present
    pub contains_env_begin: bool,
    /// Whether an end environment pattern is present
    pub contains_env_end: bool,
    /// Whether an item pattern is present
    pub contains_item: bool,
}

impl Pattern {
    /// Check if a string contains patterns
    pub fn new(s: &str) -> Self {
        Self {
            contains_env_begin: s.contains(ENV_BEGIN),
            contains_env_end: s.contains(ENV_END),
            contains_item: s.contains(ITEM),
        }
    }
}

/// Ensure that the indentation returns to zero at the end of the file
fn indents_return_to_zero(state: &State) -> bool {
    #![allow(clippy::missing_const_for_fn)]
    state.indent.actual == 0
}
