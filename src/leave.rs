/*
use crate::logging::*;
use crate::regexes::*;
use log::Level::Warn;

#[derive(Debug)]
pub struct Leave {
    pub actual: i8,
    pub visual: bool,
}

impl Leave {
    pub fn new() -> Self {
        Leave {
            actual: 0,
            visual: false,
        }
    }
}

pub fn get_leave(
    line: &str,
    linum: usize,
    leave: Leave,
    file: &str,
    logs: &mut Vec<Log>,
    pass: Option<usize>,
    warn: bool,
) -> Leave {
    let diff = get_leave_diff(line);
    let actual = leave.actual + diff;
    let visual = actual > 0 && leave.actual > 0;

    if warn && (actual < 0) {
        record_log(
            logs,
            Warn,
            pass,
            file.to_string(),
            Some(linum),
            Some(line.to_string()),
            "Leave count is negative.".to_string(),
        );
    }

    Leave { actual, visual }
}

fn get_leave_diff(line: &str) -> i8 {
    if RE_ENV_BEGIN.is_match(line) {
        for re_leave_begin in RE_LEAVES_BEGIN.iter() {
            if re_leave_begin.is_match(line) {
                return 1;
            };
        }
    } else if RE_ENV_END.is_match(line) {
        for re_leave_end in RE_LEAVES_END.iter() {
            if re_leave_end.is_match(line) {
                return -1;
            };
        }
    }
    0
}
*/
