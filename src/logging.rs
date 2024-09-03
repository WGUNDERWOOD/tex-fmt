//! Utilities for logging

use crate::colors::*;
use crate::Cli;
use env_logger::Builder;
use log::Level;
use log::Level::{Debug, Error, Info, Trace, Warn};
use log::LevelFilter;
use std::cmp::Reverse;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

/// Holds a log entry
#[derive(Debug)]
pub struct Log {
    /// Log entry level
    pub level: Level,
    /// Time when the entry was logged
    pub time: Instant,
    /// File name associated with the entry
    pub file: String,
    /// Line number in the formatted file
    pub linum_new: Option<usize>,
    /// Line number in the original file
    pub linum_old: Option<usize>,
    /// Line content
    pub line: Option<String>,
    /// Entry-specific message
    pub message: String,
}

/// Append a log to the logs list
fn record_log(
    logs: &mut Vec<Log>,
    level: Level,
    file: &str,
    linum_new: Option<usize>,
    linum_old: Option<usize>,
    line: Option<String>,
    message: &str,
) {
    let log = Log {
        level,
        time: Instant::now(),
        file: file.to_string(),
        linum_new,
        linum_old,
        line,
        message: message.to_string(),
    };
    logs.push(log);
}

/// Append a file log to the logs list
pub fn record_file_log(
    logs: &mut Vec<Log>,
    level: Level,
    file: &str,
    message: &str,
) {
    record_log(logs, level, file, None, None, None, message);
}

/// Append a line log to the logs list
pub fn record_line_log(
    logs: &mut Vec<Log>,
    level: Level,
    file: &str,
    linum_new: usize,
    linum_old: usize,
    line: &str,
    message: &str,
) {
    record_log(
        logs,
        level,
        file,
        Some(linum_new),
        Some(linum_old),
        Some(line.to_string()),
        message,
    );
}

/// Get the formatting style of a log level
fn get_log_style(log_level: Level) -> String {
    match log_level {
        Info => CYAN.to_string(),
        Warn => YELLOW.to_string(),
        Error => RED.to_string(),
        Trace => GREEN.to_string(),
        Debug => panic!(),
    }
}

/// Parse the log level from the command line arguments
const fn get_log_level(args: &Cli) -> LevelFilter {
    if args.trace {
        LevelFilter::Trace
    } else if args.verbose {
        LevelFilter::Info
    } else if args.quiet {
        LevelFilter::Error
    } else {
        LevelFilter::Warn
    }
}

/// Start the logger
pub fn init_logger(args: &Cli) {
    Builder::new()
        .filter_level(get_log_level(args))
        .format(|buf, record| {
            writeln!(
                buf,
                "{}{}:{} {}",
                get_log_style(record.level()),
                record.level(),
                RESET,
                record.args()
            )
        })
        .init();
}

/// Display all of the logs collected
pub fn print_logs(mut logs: Vec<Log>) {
    logs.sort_by_key(|l| {
        (
            l.level,
            l.linum_new,
            l.linum_old,
            l.message.clone(),
            Reverse(l.time),
        )
    });
    logs.dedup_by(|a, b| {
        (a.level, a.linum_new, a.linum_old, &a.message)
            == (b.level, b.linum_new, b.linum_old, &b.message)
    });
    logs.sort_by_key(|l| l.time);

    for log in logs {
        let linum_new = log
            .linum_new
            .map_or_else(String::new, |i| format!("Line {i} "));

        let linum_old = log
            .linum_old
            .map_or_else(String::new, |i| format!("({i}). "));

        let line = log
            .line
            .as_ref()
            .map_or_else(String::new, |l| l.trim_start().to_string());

        let log_string = format!(
            "{}tex-fmt {}{}: {}{}{}{}{} {}{}",
            PINK,
            PURPLE,
            Path::new(&log.file).file_name().unwrap().to_str().unwrap(),
            WHITE,
            linum_new,
            linum_old,
            YELLOW,
            log.message,
            RESET,
            line,
        );

        match log.level {
            Error => log::error!("{}", log_string),
            Warn => log::warn!("{}", log_string),
            Info => log::info!("{}", log_string),
            Trace => log::trace!("{}", log_string),
            Debug => panic!(),
        }
    }
}
