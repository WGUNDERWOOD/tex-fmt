use crate::Builder;
use crate::Cli;
use log::Level;
use log::LevelFilter;
use std::io::Write;

const RED: &str = "\x1b[31m\x1b[1m";
const YELLOW: &str = "\x1b[33m\x1b[1m";
const RESET: &str = "\x1b[00m\x1b[0m";

fn get_log_style(log_level: Level) -> String {
    match log_level {
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
