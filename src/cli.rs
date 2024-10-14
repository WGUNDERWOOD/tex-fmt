//! Utilities for reading the command line arguments

use crate::logging::*;
use clap::Parser;
use log::Level::Error;
use log::LevelFilter;

/// Command line arguments
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, short, help = "Check formatting, do not modify files")]
    pub check: bool,
    #[arg(long, short, help = "Print to STDOUT, do not modify files")]
    pub print: bool,
    #[arg(long, short, help = "Keep lines, do not wrap")]
    pub keep: bool,
    #[arg(long, short, help = "Show info log messages")]
    pub verbose: bool,
    #[arg(long, short, help = "Hide warning messages")]
    pub quiet: bool,
    #[arg(long, short, help = "Show trace log messages")]
    pub trace: bool,
    #[arg(help = "List of files to be formatted")]
    pub files: Vec<String>,
    #[arg(
        long,
        short,
        help = "Process STDIN as a single file, output formatted text to STDOUT"
    )]
    pub stdin: bool,
    #[arg(
        long,
        help = "Number of spaces to use as tab size",
        default_value_t = 2
    )]
    pub tab: i8,
    #[arg(long, help = "Use tabs instead of spaces for indentation")]
    pub usetabs: bool,
    #[arg(long, help = "Line length for wrapping", default_value_t = 80)]
    pub wrap: u8,
    #[clap(skip)]
    pub wrap_min: u8,
}

impl Cli {
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
