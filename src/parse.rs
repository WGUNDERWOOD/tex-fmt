//! Utilities for reading the command line arguments

use crate::logging::*;
use crate::regexes::*;
use clap::Parser;
use log::Level::Error;
use log::LevelFilter;
use std::fs;

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
    #[arg(help = "List of files that should be formatted")]
    pub files: Vec<String>,
    #[arg(
        long,
        short,
        help = "Process STDIN as a single file and output formatted text to STDOUT"
    )]
    pub stdin: bool,
}

impl Cli {
    /// Get the log level.
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
    pub fn resolve(&mut self) {
        use std::process::exit;

        if self.trace {
            self.verbose = true;
        }
        self.print |= self.stdin;

        if !self.stdin && self.files.is_empty() {
            log::error!("No files specified, either provide at least one filename or set the --stdin flag.");
            exit(1);
        }
        if self.stdin && !self.files.is_empty() {
            log::warn!("Provided file name(s) will be ignored when using the --stdin flag.");
            self.files.clear();
        }
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
        }
    }
}

/// Add a missing extension and read the file
pub fn read(file: &str, logs: &mut Vec<Log>) -> Option<(String, String)> {
    // check if file has an accepted extension
    let has_ext = EXTENSIONS.iter().any(|e| file.ends_with(e));
    // if no valid extension, try adding .tex
    let mut new_file = file.to_owned();
    if !has_ext {
        new_file.push_str(".tex");
    };
    if let Ok(text) = fs::read_to_string(&new_file) {
        return Some((new_file, text));
    }
    if has_ext {
        record_file_log(logs, Error, file, "Could not open file.");
    } else {
        record_file_log(logs, Error, file, "File type invalid.");
    }
    None
}

/// Attempts to read from STDIN and return the filename `<STDIN>` and text.
pub fn read_stdin(logs: &mut Vec<Log>) -> Option<(String, String)> {
    use std::io::Read;

    let mut text = String::new();
    match std::io::stdin().read_to_string(&mut text) {
        Ok(bytes) => {
            record_file_log(
                logs,
                log::Level::Trace,
                "<STDIN>",
                &format!("Read {bytes} bytes."),
            );
            Some((String::from("<STDIN>"), text))
        }
        Err(e) => {
            record_file_log(
                logs,
                Error,
                "<STDIN>",
                &format!("Could not read from STDIN: {e}"),
            );
            None
        }
    }
}
