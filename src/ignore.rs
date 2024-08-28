/*
use crate::logging::*;
use log::Level::Warn;

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
    linum: usize,
    ignore: Ignore,
    file: &str,
    logs: &mut Vec<Log>,
    pass: Option<usize>,
    warn: bool,
) -> Ignore {
    let skip = contains_ignore_skip(line);
    let begin = contains_ignore_begin(line);
    let end = contains_ignore_end(line);
    let actual: bool;
    let visual: bool;

    if skip {
        actual = ignore.actual;
        visual = true;
    } else if begin {
        actual = true;
        visual = true;
        if warn && ignore.actual {
            record_log(
                logs,
                Warn,
                pass,
                file.to_string(),
                Some(linum),
                Some(line.to_string()),
                "Cannot begin ignore block:".to_string(),
            );
        }
    } else if end {
        actual = false;
        visual = true;
        if warn && !ignore.actual {
            record_log(
                logs,
                Warn,
                pass,
                file.to_string(),
                Some(linum),
                Some(line.to_string()),
                "No ignore block to end:".to_string(),
            );
        }
    } else {
        actual = ignore.actual;
        visual = ignore.actual;
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
*/
