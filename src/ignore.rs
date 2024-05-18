use crate::logging::*;
use log::Level::Error;

//const IG_STARTS: [&str; 1] = ["\\begin{verbatim}"];
//const IG_ENDS: [&str; 1] = ["\\end{verbatim}"];

pub struct Ignore {
    skip: bool,
    block: bool,
}

impl Ignore {
    pub fn new() -> Self {
        Ignore {
            skip: false,
            block: false,
        }
    }
}

pub fn get_ignore(
    line: &str,
    linum: usize,
    ignore: Ignore,
    filename: &str,
    logs: &mut Vec<Log>,
    pass: Option<usize>,
    warn: bool,
) -> Ignore {
    let skip = contains_ignore_skip(line);
    let start = contains_ignore_start(line);
    let end = contains_ignore_end(line);
    let mut block = ignore.block;

    if !ignore.block {
        // not currently in ignore block
        if start {
            block = true
        }
        if end && warn {
            record_log(
                logs,
                Error,
                pass,
                filename.to_string(),
                Some(linum),
                Some(line.to_string()),
                "No ignore block to end:".to_string(),
            );
        }
    } else {
        // currently in ignore block
        if start && warn {
            record_log(
                logs,
                Error,
                pass,
                filename.to_string(),
                Some(linum),
                Some(line.to_string()),
                "Cannot start ignore block:".to_string(),
            );
        }
        if end {
            block = false
        }
    }

    Ignore { skip, block }
}

pub fn is_ignored(ignore: &Ignore) -> bool {
    ignore.skip || ignore.block
}

fn contains_ignore_skip(line: &str) -> bool {
    line.ends_with("% tex-fmt: skip")
}

fn contains_ignore_start(line: &str) -> bool {
    line.ends_with("% tex-fmt: off")
}

fn contains_ignore_end(line: &str) -> bool {
    line.ends_with("% tex-fmt: on")
}
