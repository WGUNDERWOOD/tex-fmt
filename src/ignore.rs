//! Utilities for ignoring/skipping source lines

use crate::format::State;
use crate::logging::{record_line_log, Log};
use log::Level::Warn;
use std::path::Path;

/// Information on the ignored state of a line
#[derive(Clone, Debug)]
pub struct Ignore {
    /// Whether the line is in an ignore block
    pub actual: bool,
    /// Whether the line should be ignored/skipped
    pub visual: bool,
}

impl Ignore {
    /// Construct a new ignore state
    #[must_use]
    pub const fn new() -> Self {
        Self {
            actual: false,
            visual: false,
        }
    }
}

impl Default for Ignore {
    fn default() -> Self {
        Self::new()
    }
}

/// Determine whether a line should be ignored
pub fn get_ignore(
    line: &str,
    state: &State,
    logs: &mut Vec<Log>,
    file: &Path,
    warn: bool,
) -> Ignore {
    let skip = contains_ignore_skip(line);
    let begin = contains_ignore_begin(line);
    let end = contains_ignore_end(line);
    let actual: bool;
    let visual: bool;

    if skip {
        actual = state.ignore.actual;
        visual = true;
    } else if begin {
        actual = true;
        visual = true;
        if warn && state.ignore.actual {
            record_line_log(
                logs,
                Warn,
                file,
                state.linum_new,
                state.linum_old,
                line,
                "Cannot begin ignore block:",
            );
        }
    } else if end {
        actual = false;
        visual = true;
        if warn && !state.ignore.actual {
            record_line_log(
                logs,
                Warn,
                file,
                state.linum_new,
                state.linum_old,
                line,
                "No ignore block to end.",
            );
        }
    } else {
        actual = state.ignore.actual;
        visual = state.ignore.actual;
    }

    Ignore { actual, visual }
}

/// Check if a line contains a skip directive
fn contains_ignore_skip(line: &str) -> bool {
    line.ends_with("% tex-fmt: skip")
}

/// Check if a line contains the start of an ignore block
fn contains_ignore_begin(line: &str) -> bool {
    line.ends_with("% tex-fmt: off")
}

/// Check if a line contains the end of an ignore block
fn contains_ignore_end(line: &str) -> bool {
    line.ends_with("% tex-fmt: on")
}
