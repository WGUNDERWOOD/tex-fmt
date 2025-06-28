//! Core methodology for formatting a file

use crate::args::{Args, TabChar};
use crate::ignore::{get_ignore, Ignore};
use crate::indent::{apply_indent, calculate_indent, Indent};
use crate::logging::{record_file_log, Log};
use crate::read::{read, read_stdin};
use crate::regexes::{ENV_BEGIN, ENV_END, ITEM, RE_SPLITTING, VERB};
use crate::subs;
use crate::verbatim::{get_verbatim, Verbatim};
use crate::wrap::{apply_wrap, needs_wrap};
use crate::write::process_output;
use crate::LINE_END;
use log::Level::{Info, Warn};
use std::iter::zip;
use std::path::PathBuf;

/// Central function to format a file
///
/// # Panics
///
/// This function panics upon an unrecoverable formatting error
#[allow(clippy::too_many_lines)]
pub fn format_file(
    old_text: &str,
    file: &PathBuf,
    args: &Args,
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
    let indent_char = match args.tabchar {
        TabChar::Tab => "\t",
        TabChar::Space => " ",
    };

    // Get special environments
    let lists_begin = get_begins(&args.lists);
    let lists_end = get_ends(&args.lists);
    let verbatims_begin = get_begins(&args.verbatims);
    let verbatims_end = get_ends(&args.verbatims);
    let no_indent_envs_begin = get_begins(&args.no_indent_envs);
    let no_indent_envs_end = get_ends(&args.no_indent_envs);

    loop {
        if let Some((linum_old, mut line)) = queue.pop() {
            // Read the patterns present on this line.
            let pattern = Pattern::new(&line);

            // Temporary state for working on this line.
            let mut temp_state = state.clone();

            // Update the state with the line number from the queue.
            temp_state.linum_old = linum_old;

            // If the line should not be ignored...
            if !set_ignore_and_report(
                &line,
                &mut temp_state,
                logs,
                file,
                &pattern,
                &verbatims_begin,
                &verbatims_end,
            ) {
                // Check if the line should be split because of a pattern
                // that should begin on a new line.
                if subs::needs_split(&line, &pattern) {
                    // Split the line into two...
                    let (this_line, next_line) =
                        subs::split_line(&line, &temp_state, file, args, logs);
                    // ... and queue the second part for formatting.
                    queue.push((linum_old, next_line.to_string()));
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
                    &lists_begin,
                    &lists_end,
                    &no_indent_envs_begin,
                    &no_indent_envs_end,
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
                        &pattern,
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

    // Warn about indents not returning to zero
    if !indents_return_to_zero(&state) {
        let msg = format!(
            "Indent does not return to zero. Last non-indented line is line {}",
            state.linum_last_zero_indent
        );
        record_file_log(logs, Warn, file, &msg);
    }

    // Warn about first negative indent
    if let Some(n) = state.linum_first_negative_indent {
        let msg = format!(
            "Negative indents. First negatively indented line is line {n}",
        );
        record_file_log(logs, Warn, file, &msg);
    }

    new_text = subs::remove_trailing_spaces(&new_text);
    new_text = subs::remove_trailing_blank_lines(&new_text);
    record_file_log(logs, Info, file, "Formatting complete.");
    new_text
}

fn get_begins(v: &[String]) -> Vec<String> {
    v.iter().map(|l| format!("\\begin{{{l}}}")).collect()
}

fn get_ends(v: &[String]) -> Vec<String> {
    v.iter().map(|l| format!("\\end{{{l}}}")).collect()
}

/// Sets the `ignore` and `verbatim` flags in the given [State] based on
/// `line` and returns whether `line` should be ignored by formatting.
fn set_ignore_and_report(
    line: &str,
    temp_state: &mut State,
    logs: &mut Vec<Log>,
    file: &PathBuf,
    pattern: &Pattern,
    verbatims_begin: &[String],
    verbatims_end: &[String],
) -> bool {
    temp_state.ignore = get_ignore(line, temp_state, logs, file, true);
    temp_state.verbatim = get_verbatim(
        line,
        temp_state,
        logs,
        file,
        true,
        pattern,
        verbatims_begin,
        verbatims_end,
    );

    temp_state.verbatim.visual || temp_state.ignore.visual
}

/// Cleans the given text by removing extra line breaks and trailing spaces,
/// and also tabs if they shouldn't be used.
fn clean_text(text: &str, args: &Args) -> String {
    let mut text = subs::remove_extra_newlines(text);

    if args.tabchar != TabChar::Tab {
        text = subs::remove_tabs(&text, args);
    }

    text = subs::remove_trailing_spaces(&text);

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
    /// Line number in the new file of the first negatively indented line
    pub linum_first_negative_indent: Option<usize>,
}

impl State {
    /// Construct a new default state
    #[must_use]
    pub const fn new() -> Self {
        Self {
            linum_old: 1,
            linum_new: 1,
            ignore: Ignore::new(),
            indent: Indent::new(),
            verbatim: Verbatim::new(),
            linum_last_zero_indent: 1,
            linum_first_negative_indent: None,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// Record whether a line contains certain patterns to avoid recomputing
#[allow(clippy::struct_excessive_bools)]
pub struct Pattern {
    /// Whether a begin environment pattern is present
    pub contains_env_begin: bool,
    /// Whether an end environment pattern is present
    pub contains_env_end: bool,
    /// Whether an item pattern is present
    pub contains_item: bool,
    /// Whether a splitting pattern is present
    pub contains_splitting: bool,
    /// Whether a comment is present
    pub contains_comment: bool,
    /// Whether a verb environment is present
    pub contains_verb: bool,
}

impl Pattern {
    /// Check if a string contains patterns
    #[must_use]
    pub fn new(s: &str) -> Self {
        // If splitting does not match, most patterns are not present
        if RE_SPLITTING.is_match(s) {
            Self {
                contains_env_begin: s.contains(ENV_BEGIN),
                contains_env_end: s.contains(ENV_END),
                contains_item: s.contains(ITEM),
                contains_splitting: true,
                contains_comment: s.contains('%'),
                contains_verb: s.contains(VERB),
            }
        } else {
            Self {
                contains_env_begin: false,
                contains_env_end: false,
                contains_item: false,
                contains_splitting: false,
                contains_comment: s.contains('%'),
                contains_verb: s.contains(VERB),
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
        let stdin_path = PathBuf::from("<stdin>");
        if let Some(text) = read_stdin(logs) {
            let new_text = format_file(&text, &stdin_path, args, logs);
            exit_code =
                process_output(args, &stdin_path, &text, &new_text, logs);
        } else {
            exit_code = 1;
        }
    } else {
        for file in &args.files {
            if let Some(text) = read(file, logs) {
                let new_text = format_file(&text, &file, args, logs);
                exit_code |=
                    process_output(args, &file, &text, &new_text, logs);
            } else {
                exit_code = 1;
            }
        }
    }
    exit_code
}
