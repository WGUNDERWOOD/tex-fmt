use crate::colors::*;
//use crate::logging::*;
//use log::Level::Warn;

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
    filename: &str,
    prev_ignore: Ignore,
    logs: &mut Vec<Log>,
) -> Ignore {
    let skip = contains_ignore_skip(line);
    let start = contains_ignore_start(line);
    let end = contains_ignore_end(line);
    let mut block = prev_ignore.block;

    if !prev_ignore.block {
        // not currently in ignore block
        if start {
            block = true
        }
        if end {
            log::error!(
                "Line {}: no ignore block to end: {}{:.50}...",
                i,
                WHITE,
                line
            );
            let log = Log {
                level: Warn,
                linum,
                message,
                filename: filename.to_string(),
            };
            record_log(logs, log);
        }
    } else {
        // currently in ignore block
        if start {
            log::error!(
                "Line {}: cannot start ignore block \
                        before ending previous block: {}{:.50}...",
                i,
                WHITE,
                line
            );
            let log = Log {
                level: Warn,
                linum,
                message,
                filename: filename.to_string(),
            };
            record_log(logs, log);
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
