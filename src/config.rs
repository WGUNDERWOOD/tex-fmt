//! Configuration for tex-fmt.
//! After reading the command line arguments and all config files the struct Config should be created and used for configuring tex-fmt

use crate::logging::*;
use log::Level::Error;
use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub check: bool,
    pub print: bool,
    pub keep: bool,
    pub verbose: bool,
    pub stdin: bool,
    pub quiet: bool,
    pub trace: bool,
    pub files: Vec<String>,
    pub tab: i8, // NOTE: why is this an i8?
    pub usetabs: bool,
    pub wrap: u8,
    pub wrap_min: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            check: false,
            print: false,
            keep: false,
            verbose: false,
            quiet: false,
            trace: false,
            files: vec![],
            stdin: false,
            tab: 2,
            usetabs: false,
            wrap: 80,
            wrap_min: 70,
        }
    }
}

impl Config {
    /// Get the log level
    pub const fn log_level(&self) -> LevelFilter {
        if self.trace {
            LevelFilter::Trace
        } else if self.verbose {
            LevelFilter::Info
        } else if self.quiet {
            LevelFilter::Error
        } else {
            LevelFilter::Warn
        }
    }

    /// Ensure the provided arguments are consistent
    pub fn resolve(&mut self, logs: &mut Vec<Log>) -> u8 {
        let mut exit_code = 0;
        self.verbose |= self.trace;
        self.print |= self.stdin;
        self.wrap_min = if self.wrap >= 50 {
            self.wrap - 10
        } else {
            self.wrap
        };

        if !self.stdin && self.files.is_empty() {
            record_file_log(
                logs,
                Error,
                "",
                "No files specified. Either provide filenames or provide --stdin.",
            );
            exit_code = 1;
        }
        if self.stdin && !self.files.is_empty() {
            record_file_log(
                logs,
                Error,
                "",
                "Do not provide file name(s) when using --stdin.",
            );
            exit_code = 1;
        }
        exit_code
    }

    #[cfg(test)]
    pub const fn new() -> Self {
        Self {
            check: false,
            print: false,
            keep: false,
            verbose: false,
            stdin: false,
            quiet: false,
            trace: false,
            files: Vec::<String>::new(),
            tab: 2,
            usetabs: false,
            wrap: 80,
            wrap_min: 70,
        }
    }
}
