use crate::colors::*;
use crate::print::*;
use crate::Cli;
use env_logger::Builder;
use log::Level;
use log::LevelFilter;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct Log {
    pub level: Level,
    pub linum: usize,
    pub message: String,
    pub filename: String,
}

pub fn record_log(logs: &mut Vec<Log>, log: Log) {
    logs.push(log);
}

fn get_log_style(log_level: Level) -> String {
    match log_level {
        Level::Info => CYAN.to_string(),
        Level::Warn => YELLOW.to_string(),
        Level::Error => RED.to_string(),
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

pub fn print_logs(logs: &mut Vec<Log>, filename: &str) {
    logs.sort_by_key(|l| l.linum);
    if !logs.is_empty() {
        print_filename(filename);
        for log in logs {
            match log.level {
                Level::Error => log::error!("{}", log.message),
                Level::Warn => log::warn!("{}", log.message),
                Level::Info => log::info!("{}", log.message),
                Level::Debug => log::debug!("{}", log.message),
                Level::Trace => log::trace!("{}", log.message),
            }
        }
    }
}
