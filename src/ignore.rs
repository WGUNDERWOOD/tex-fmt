use crate::colors::*;

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

pub fn get_ignore(line: &str, i: usize, prev_ignore: Ignore) -> Ignore {
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
            log::warn!(
                "No ignore block to end on line {}: {}{:.50}...",
                i,
                WHITE,
                line
            );
        }
    } else {
        // currently in ignore block
        if start {
            log::warn!(
                "Cannot start ignore block on line {} before ending previous block: {}{:.50}...",
                i,
                WHITE,
                line
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
