//! Utilities for performing text substitutions

use crate::args::Args;
use crate::comments::find_comment_index;
use crate::format::{Pattern, State};
use crate::logging::{record_line_log, Log};
use crate::regexes;
use crate::LINE_END;
use log::Level;
use log::LevelFilter;

/// Remove multiple line breaks
#[must_use]
pub fn remove_extra_newlines(text: &str) -> String {
    let double_line_end = format!("{LINE_END}{LINE_END}");
    regexes::RE_NEWLINES
        .replace_all(text, double_line_end)
        .to_string()
}

/// Replace tabs with spaces
#[must_use]
pub fn remove_tabs(text: &str, args: &Args) -> String {
    let replace = (0..args.tabsize).map(|_| " ").collect::<String>();
    text.replace('\t', &replace)
}

/// Remove trailing spaces from line endings
#[must_use]
pub fn remove_trailing_spaces(text: &str) -> String {
    regexes::RE_TRAIL.replace_all(text, LINE_END).to_string()
}

/// Remove trailing blank lines from file
#[must_use]
pub fn remove_trailing_blank_lines(text: &str) -> String {
    let mut new_text = text.trim_end().to_string();
    new_text.push_str(LINE_END);
    new_text
}

/// Check if line contains content which be split onto a new line
///
/// # Panics
///
/// This function should not panic as we already check for a regex match
/// before finding the match index and unwrapping.
#[must_use]
pub fn needs_split(line: &str, pattern: &Pattern) -> bool {
    // Don't split anything if the line contains a \verb|...|
    if pattern.contains_verb && line.contains(regexes::VERB) {
        return false;
    }

    // Check if we should format this line and if we've matched an environment.
    let contains_splittable_env = pattern.contains_splitting
        && regexes::RE_SPLITTING_SHARED_LINE.is_match(line);

    // If we're not ignoring and we've matched an environment ...
    if contains_splittable_env {
        // ... return `true` if the comment index is `None`
        // (which implies the split point must be in text), otherwise
        // compare the index of the comment with the split point.
        find_comment_index(line, pattern).is_none_or(|comment_index| {
            if regexes::RE_SPLITTING_SHARED_LINE_CAPTURE
                .captures(line)
                .unwrap() // Matched split point so no panic.
                .get(2)
                .unwrap() // Regex has 4 groups so index 2 is in bounds.
                .start()
                > comment_index
            {
                // If split point is past the comment index, don't split.
                false
            } else {
                // Otherwise, split point is before comment and we do split.
                true
            }
        })
    } else {
        // If ignoring or didn't match an environment, don't need a new line.
        false
    }
}

/// Ensure lines are split correctly.
///
/// Returns a tuple containing:
/// 1. a reference to the line that was given, shortened because of the split
/// 2. a reference to the part of the line that was split
///
/// # Panics
///
/// This function should not panic as all regexes are validated.
pub fn split_line<'a>(
    line: &'a str,
    state: &State,
    file: &str,
    args: &Args,
    logs: &mut Vec<Log>,
) -> (&'a str, &'a str) {
    let captures = regexes::RE_SPLITTING_SHARED_LINE_CAPTURE
        .captures(line)
        .unwrap();

    let (line, [prev, rest, _]) = captures.extract();

    if args.verbosity == LevelFilter::Trace {
        record_line_log(
            logs,
            Level::Trace,
            file,
            state.linum_new,
            state.linum_old,
            line,
            "Placing environment on new line.",
        );
    }
    (prev, rest)
}
