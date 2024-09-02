use crate::format::*;
use crate::logging::*;
use crate::regexes::*;
use log::Level::Warn;

#[derive(Clone, Debug)]
pub struct Leave {
    pub actual: i8,
    pub visual: bool,
}

impl Leave {
    pub const fn new() -> Self {
        Self {
            actual: 0,
            visual: false,
        }
    }
}

pub fn get_leave(
    line: &str,
    state: &State,
    logs: &mut Vec<Log>,
    file: &str,
    warn: bool,
) -> Leave {
    let diff = get_leave_diff(line);
    let actual = state.leave.actual + diff;
    let visual = actual > 0 && state.leave.actual > 0;

    if warn && (actual < 0) {
        record_line_log(
            logs,
            Warn,
            file,
            state.linum_new,
            state.linum_old,
            line,
            "Leave count is negative.",
        );
    }

    Leave { actual, visual }
}

fn get_leave_diff(line: &str) -> i8 {
    if line.contains(ENV_BEGIN)
        && RE_LEAVES_BEGIN.iter().any(|r| r.is_match(line))
    {
        1
    } else if line.contains(ENV_END)
        && RE_LEAVES_END.iter().any(|r| r.is_match(line))
    {
        -1
    } else {
        0
    }
}
