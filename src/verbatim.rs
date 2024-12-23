//! Utilities for ignoring verbatim environments

use crate::format::*;
use crate::logging::*;
use crate::pattern::Pattern;
use crate::regexes::*;
use log::Level::Warn;

/// Information on the verbatim state of a line
#[derive(Clone, Debug)]
pub struct Verbatim {
    /// The verbatim depth of a line
    pub actual: i8,
    /// Whether the line is in a verbatim environment
    pub visual: bool,
}

impl Verbatim {
    /// Construct a new verbatim state
    pub const fn new() -> Self {
        Self {
            actual: 0,
            visual: false,
        }
    }
}

/// Determine whether a line is in a verbatim environment
pub fn get_verbatim(
    line: &str,
    state: &State,
    logs: &mut Vec<Log>,
    file: &str,
    warn: bool,
    pattern: &Pattern,
) -> Verbatim {
    let diff = get_verbatim_diff(line, pattern);
    let actual = state.verbatim.actual + diff;
    let visual = actual > 0 || state.verbatim.actual > 0;

    if warn && (actual < 0) {
        record_line_log(
            logs,
            Warn,
            file,
            state.linum_new,
            state.linum_old,
            line,
            "Verbatim count is negative.",
        );
    }

    Verbatim { actual, visual }
}

/// Calculate total verbatim depth change
fn get_verbatim_diff(line: &str, pattern: &Pattern) -> i8 {
    if pattern.contains_env_begin
        && VERBATIMS_BEGIN.iter().any(|r| line.contains(r))
    {
        1
    } else if pattern.contains_env_end
        && VERBATIMS_END.iter().any(|r| line.contains(r))
    {
        -1
    } else {
        0
    }
}
