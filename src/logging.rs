use crate::Builder;
use crate::Cli;
use std::io::Write;

const RED: &str = "\x1b[31m\x1b[1m";
const YELLOW: &str = "\x1b[33m\x1b[1m";
const RESET: &str = "\x1b[00m\x1b[0m";

fn get_log_style(log_level: log::Level) -> String {
    match log_level {
        log::Level::Warn => YELLOW.to_string(),
        log::Level::Error => RED.to_string(),
        _ => panic!(),
    }
}

pub fn init_logger(args: &Cli) {
    let log_level = match args.verbose {
        true => log::LevelFilter::Warn,
        false => log::LevelFilter::Error,
    };
    Builder::new()
        .filter_level(log_level)
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
