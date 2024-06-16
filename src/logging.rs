use crate::colors::*;
use crate::Cli;
use env_logger::Builder;
use log::Level;
use log::Level::{Error, Info, Trace, Warn};
use log::LevelFilter;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

#[derive(Debug)]
pub struct Log {
    pub level: Level,
    pub pass: Option<usize>,
    pub time: Instant,
    pub file: String,
    pub linum: Option<usize>,
    pub line: Option<String>,
    pub message: String,
}

pub fn record_log(
    logs: &mut Vec<Log>,
    level: Level,
    pass: Option<usize>,
    file: String,
    linum: Option<usize>,
    line: Option<String>,
    message: String,
) {
    let log = Log {
        level,
        pass,
        time: Instant::now(),
        file,
        linum,
        line,
        message,
    };
    logs.push(log);
}

fn get_log_style(log_level: Level) -> String {
    match log_level {
        Info => CYAN.to_string(),
        Warn => YELLOW.to_string(),
        Error => RED.to_string(),
        Trace => GREEN.to_string(),
        _ => panic!(),
    }
}

fn get_log_level(args: &Cli) -> LevelFilter {
    if args.trace {
        LevelFilter::Trace
    } else if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    }
}

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

pub fn print_logs(args: &Cli, mut logs: Vec<Log>) {
    if get_log_level(args) == LevelFilter::Warn && !logs.is_empty() {
        let max_pass = &logs.iter().map(|l| l.pass).max().unwrap();
        logs.retain(|l| l.pass == *max_pass || l.pass.is_none());
    }

    logs.sort_by_key(|l| l.time);

    for log in logs {
        let linum = match log.linum {
            // linums start from 1
            Some(i) => format!("Line {}. ", i + 1),
            None => "".to_string(),
        };

        let line = match &log.line {
            Some(l) => l.trim_start().to_string(),
            None => "".to_string(),
        };

        let log_string = format!(
            "{}tex-fmt {}{}: {}{}{}{} {}{}",
            PINK,
            PURPLE,
            Path::new(&log.file)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
            WHITE,
            linum,
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
            _ => panic!(),
        }
    }
}
