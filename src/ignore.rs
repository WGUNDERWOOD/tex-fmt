use crate::format::*;
use crate::logging::*;
use log::Level::Warn;

#[derive(Clone, Debug)]
pub struct Ignore {
    pub actual: bool,
    pub visual: bool,
}

impl Ignore {
    pub fn new() -> Self {
        Ignore {
            actual: false,
            visual: false,
        }
    }
}

pub fn get_ignore(
    line: &str,
    state: &State,
    logs: &mut Vec<Log>,
    file: &str,
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

fn contains_ignore_skip(line: &str) -> bool {
    line.ends_with("% tex-fmt: skip")
}

fn contains_ignore_begin(line: &str) -> bool {
    line.ends_with("% tex-fmt: off")
}

fn contains_ignore_end(line: &str) -> bool {
    line.ends_with("% tex-fmt: on")
}
