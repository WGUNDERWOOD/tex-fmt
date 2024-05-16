use crate::colors::*;
use crate::print::*;
use crate::Cli;
use env_logger::Builder;
use log::Level;
use log::Level::{Debug, Error, Info, Trace, Warn};
use log::LevelFilter;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::io::Write;
use std::sync::Mutex;

pub static LOGS: Lazy<Mutex<HashSet<Log>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Log {
    level: Level,
    linum: usize,
    message: String,
}

fn get_log_style(log_level: Level) -> String {
    match log_level {
        Info => CYAN.to_string(),
        Warn => YELLOW.to_string(),
        Error => RED.to_string(),
        _ => panic!(),
    }
}

fn get_log_level(args: &Cli) -> LevelFilter {
    match args.verbose {
        true => LevelFilter::Info,
        false => LevelFilter::Warn,
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

pub fn record_log(level: Level, linum: usize, message: String) {
    let mut logs = LOGS.lock().unwrap();
    let log = Log {
        level,
        linum,
        message,
    };
    logs.insert(log);
}

pub fn print_logs(filename: &str) {
    let mut logs: Vec<Log> = vec![];
    for log in LOGS.lock().unwrap().iter() {
        logs.push(log.clone());
    }
    logs.sort_by_key(|l| l.linum);

    if !logs.is_empty() {
        print_filename(filename);
    }
    for log in logs {
        match log.level {
            Error => log::error!("{}", log.message),
            Warn => log::warn!("{}", log.message),
            Info => log::info!("{}", log.message),
            Debug => log::debug!("{}", log.message),
            Trace => log::trace!("{}", log.message),
        }
    }
}
