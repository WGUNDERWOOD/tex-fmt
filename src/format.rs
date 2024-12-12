//! Core methodology for formatting a file

use crate::args::*;
use crate::ignore::*;
use crate::indent::*;
use crate::logging::*;
use crate::read::*;
use crate::regexes::{ENV_BEGIN, ENV_END, ITEM, RE_SPLITTING};
use crate::subs::*;
use crate::verbatim::*;
use crate::wrap::*;
use crate::write::*;
use crate::LINE_END;
use log::Level::{Info, Warn};
use std::collections::VecDeque;
use std::iter::zip;

/// The default number of lines to have in the formatting queue.
///
/// Two lines are required for re-wrapping so that the line following the current one can always be accessed.
const DEFAULT_QUEUE_LENGTH: usize = 2;

/// Central function to format a file
pub fn format_file(
    old_text: &str,
    file: &str,
    args: &Args,
    logs: &mut Vec<Log>,
) -> String {
    record_file_log(logs, Info, file, "Formatting started.");

    // Clean the source file and zip its lines with line numbers
    let old_text = clean_text(old_text, args);
    let mut old_lines = zip(1.., old_text.lines());

    // Initialise
    let mut state = State::new();
    let mut queue: VecDeque<(usize, String)> = vec![].into();
    let mut new_text = String::with_capacity(2 * old_text.len());

    // Select the character used for indentation.
    let indent_char = match args.tabchar {
        TabChar::Tab => "\t",
        TabChar::Space => " ",
    };

    'main: loop {
        // Add more lines to the queue if there aren't two
        for _ in 0..DEFAULT_QUEUE_LENGTH.saturating_sub(queue.len()) {
            if let Some((linum_old, line)) = old_lines.next() {
                queue.push_back((linum_old, line.to_string()));
            }
        }

        if let Some((linum_old, mut line)) = queue.pop_front() {
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
                // Check if the line should be split because of a pattern
                // that should begin on a new line.
                if needs_split(&line, &pattern) {
                    // Split the line into two ...
                    let (this_line, next_line) =
                        split_line(&line, &temp_state, file, args, logs);
                    // ... and add the second part to the front of the queue for formatting.
                    queue.push_front((linum_old, next_line.to_string()));
                    line = this_line.to_string();
                }

                // Calculate the indent based on the current state
                // and the patterns in the line.
                let indent = calculate_indent(
                    &line,
                    &mut temp_state,
                    logs,
                    file,
                    args,
                    &pattern,
                );

                #[allow(clippy::cast_possible_wrap)]
                let indent_length =
                    usize::try_from(indent.visual * args.tabsize as i8)
                        .expect("Visual indent is non-negative.");

                // Wrap the line before applying the indent, and loop back
                // if the line needed wrapping.
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
                        queue.push_front((
                            linum_old,
                            [next_line_start, next_line].concat(),
                        ));
                        queue.push_front((linum_old, this_line.to_string()));
                        continue 'main;
                    }
                } else if let Some(rewrap_point) = can_rewrap(
                    line.trim_start(),
                    &pattern,
                    queue.front().map(|(_, next_line)| next_line.as_str()),
                    indent_length,
                    args,
                ) {
                    // Remove the next line from the queue and replace it after
                    // removing the re-wrapped text.
                    let (linum_old, next_line) = queue.pop_front().unwrap(); // Doesn't panic because we can re-wrap.

                    let trimmed_next_line = next_line.trim_start();

                    // Append the re-wrapped words to the current line
                    line = [
                        line.as_str(),
                        " ",
                        &trimmed_next_line[0..rewrap_point],
                    ]
                    .concat();

                    // Select the line left after re-wrapping
                    let next_line =
                        trimmed_next_line[rewrap_point..].trim_start();

                    // Add to the queue if there text left in the next line
                    if !next_line.is_empty() {
                        queue.push_front((linum_old, next_line.to_owned()));
                    }

                    // Push the current line in the queue for further potential
                    // re-wrapping
                    queue.push_front((linum_old, line));

                    // Continue the loop to avoid writing the current line for now
                    continue;
                }

                // Lastly, apply the indent if the line didn't need wrapping.
                line = apply_indent(&line, &indent, args, indent_char);
            }

            // Add line to new text
            state = temp_state;
            new_text.push_str(&line);
            new_text.push_str(LINE_END);
            state.linum_new += 1;
        } else {
            // If there are not lines in `queue`, then `old_lines` has been entirely consumed and it's safe to break the
            // main loop.
            break 'main;
        }
    }

    if !indents_return_to_zero(&state) {
        let msg = format!(
            "Indent does not return to zero. Last non-indented line is line {}",
            state.linum_last_zero_indent
        );
        record_file_log(logs, Warn, file, &msg);
    }

    new_text = remove_trailing_spaces(&new_text);
    record_file_log(logs, Info, file, "Formatting complete.");
    new_text
}

/// Sets the `ignore` and `verbatim` flags in the given [State] based on
/// `line` and returns whether `line` should be ignored by formatting.
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

/// Cleans the given text by removing extra line breaks and trailing spaces,
/// and also tabs if they shouldn't be used.
fn clean_text(text: &str, args: &Args) -> String {
    let mut text = remove_extra_newlines(text);

    if args.tabchar != TabChar::Tab {
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
    /// Line number in the new file of the last non-indented line
    pub linum_last_zero_indent: usize,
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
            linum_last_zero_indent: 1,
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
    /// Whether a splitting pattern is present
    pub contains_splitting: bool,
}

impl Pattern {
    /// Check if a string contains patterns
    pub fn new(s: &str) -> Self {
        // If splitting does not match, no patterns are present
        if RE_SPLITTING.is_match(s) {
            Self {
                contains_env_begin: s.contains(ENV_BEGIN),
                contains_env_end: s.contains(ENV_END),
                contains_item: s.contains(ITEM),
                contains_splitting: true,
            }
        } else {
            Self {
                contains_env_begin: false,
                contains_env_end: false,
                contains_item: false,
                contains_splitting: false,
            }
        }
    }
}

/// Ensure that the indentation returns to zero at the end of the file
const fn indents_return_to_zero(state: &State) -> bool {
    state.indent.actual == 0
}

/// Run tex-fmt with the provided arguments
pub fn run(args: &Args, logs: &mut Vec<Log>) -> u8 {
    let mut exit_code = 0;
    if args.stdin {
        if let Some((file, text)) = read_stdin(logs) {
            let new_text = format_file(&text, &file, args, logs);
            exit_code = process_output(args, &file, &text, &new_text, logs);
        } else {
            exit_code = 1;
        }
    } else {
        for file in &args.files {
            if let Some((file, text)) = read(file, logs) {
                let new_text = format_file(&text, &file, args, logs);
                exit_code = process_output(args, &file, &text, &new_text, logs);
            } else {
                exit_code = 1;
            }
        }
    }
    exit_code
}
