//! Utilities for logging

use colored::{Color, Colorize};
use env_logger::Builder;
use log::Level;
use log::Level::{Debug, Error, Info, Trace, Warn};
use log::LevelFilter;
use std::cmp::Reverse;
use std::io::Write;
use std::path::Path;
use web_time::Instant;

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

/// Get the color of a log level
const fn get_log_color(log_level: Level) -> Color {
    match log_level {
        Info => Color::Cyan,
        Warn => Color::Yellow,
        Error => Color::Red,
        Trace => Color::Green,
        Debug => panic!(),
    }
}

/// Start the logger
pub fn init_logger(level_filter: LevelFilter) {
    Builder::new()
        .filter_level(level_filter)
        .format(|buf, record| {
            writeln!(
                buf,
                "{}: {}",
                record
                    .level()
                    .to_string()
                    .color(get_log_color(record.level()))
                    .bold(),
                record.args()
            )
        })
        .init();
}

/// Display all of the logs collected
pub fn print_logs(logs: &mut Vec<Log>) {
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
        (
            a.level,
            &a.file,
            a.linum_new,
            a.linum_old,
            &a.line,
            &a.message,
        ) == (
            b.level,
            &b.file,
            b.linum_new,
            b.linum_old,
            &b.line,
            &b.message,
        )
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
            "{} {}: {}{}{} {}",
            "tex-fmt".magenta().bold(),
            match log.file.as_str() {
                "<stdin>" | "" => "<stdin>".blue().bold(),
                _ => Path::new(&log.file)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .blue()
                    .bold(),
            },
            linum_new.white().bold(),
            linum_old.white().bold(),
            log.message.yellow().bold(),
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
