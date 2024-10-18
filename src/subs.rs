//! Utilities for performing text substitutions

use crate::comments::*;
use crate::format::*;
use crate::logging::*;
use crate::regexes::*;
use crate::Cli;
use crate::LINE_END;
use log::Level::Trace;

/// Remove multiple line breaks
pub fn remove_extra_newlines(text: &str) -> String {
    let double_line_end = format!("{LINE_END}{LINE_END}");
    RE_NEWLINES.replace_all(text, double_line_end).to_string()
}

/// Replace tabs with spaces
pub fn remove_tabs(text: &str, args: &Cli) -> String {
    let replace = (0..args.tab).map(|_| " ").collect::<String>();
    text.replace('\t', &replace)
}

/// Remove trailing spaces from line endings
pub fn remove_trailing_spaces(text: &str) -> String {
    RE_TRAIL.replace_all(text, LINE_END).to_string()
}

/// Check if environment should be split onto a new line
pub fn needs_env_new_line(line: &str, pattern: &Pattern) -> bool {
    // Check if we should format this line and if we've matched an environment.
    let contains_splittable_env = (pattern.contains_env_begin
        || pattern.contains_env_end
        || pattern.contains_item)
        && (RE_ENV_BEGIN_SHARED_LINE.is_match(line)
            || RE_ENV_END_SHARED_LINE.is_match(line)
            || RE_ITEM_SHARED_LINE.is_match(line));

    // If we're not ignoring and we've matched an environment ...
    if contains_splittable_env {
        // ... return `true` if the comment index is `None`
        // (which implies the split point must be in text), otherwise
        // compare the index of the comment with the split point.
        find_comment_index(line).map_or(true, |comment_index| {
            if RE_ENV_ITEM_SHARED_LINE
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

/// Ensure LaTeX environments begin on new lines.
///
/// Returns a tuple containing:
/// 1. a reference to the line that was given, shortened because of the split
/// 2. a reference to the part of the line that was split
pub fn put_env_new_line<'a>(
    line: &'a str,
    state: &State,
    file: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> (&'a str, &'a str) {
    let captures = RE_ENV_ITEM_SHARED_LINE.captures(line).unwrap();

    let (line, [prev, rest, _]) = captures.extract();

    if args.trace {
        record_line_log(
            logs,
            Trace,
            file,
            state.linum_new,
            state.linum_old,
            line,
            "Placing environment on new line.",
        );
    }
    (prev, rest)
}
