//! Main arguments

use crate::cli::*;
use crate::config::*;
use crate::logging::*;
use crate::Log;
use log::Level;
use log::LevelFilter;
use merge::Merge;
use std::path::PathBuf;

/// Arguments passed to tex-fmt
#[derive(Debug)]
pub struct Args {
    /// Check formatting, do not modify files
    pub check: bool,
    /// Print to stdout, do not modify files
    pub print: bool,
    /// Wrap long lines
    pub wrap: bool,
    /// Verbosity level for log messages
    pub verbosity: LevelFilter,
    /// List of files to be formatted
    pub files: Vec<String>,
    /// Read from stdin and output to stdout
    pub stdin: bool,
    /// Number of characters to use as tab size
    pub tabsize: u8,
    /// Characters to use for indentation
    pub tabchar: TabChar,
    /// Maximum allowed line length
    pub wraplen: u8,
    /// Wrap lines longer than this
    pub wrapmin: u8,
    /// Path to config file
    pub config: Option<PathBuf>,
}

/// Arguments using Options to track CLI/config file/default values
#[derive(Clone, Debug, Merge)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct OptionArgs {
    pub check: Option<bool>,
    pub print: Option<bool>,
    pub wrap: Option<bool>,
    pub verbosity: Option<LevelFilter>,
    #[merge(strategy = merge::vec::append)]
    pub files: Vec<String>,
    pub stdin: Option<bool>,
    pub tabsize: Option<u8>,
    pub tabchar: Option<TabChar>,
    pub wraplen: Option<u8>,
    pub wrapmin: Option<u8>,
    pub config: Option<PathBuf>,
}

/// Character to use for indentation
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum TabChar {
    Tab,
    Space,
}

impl Default for OptionArgs {
    fn default() -> Self {
        Self {
            check: Some(false),
            print: Some(false),
            wrap: Some(true),
            verbosity: Some(LevelFilter::Warn),
            files: vec![],
            stdin: Some(false),
            tabsize: Some(2),
            tabchar: Some(TabChar::Space),
            wraplen: Some(80),
            wrapmin: Some(70),
            config: None,
        }
    }
}

/// Get all arguments from CLI, config file, and defaults, and merge them
pub fn get_args() -> Args {
    let mut args = get_cli_args();
    let config_args = get_config_args(&args);
    if let Some(c) = config_args {
        args.merge(c);
    }
    args.merge(OptionArgs::default());
    Args::from(args)
}

impl Args {
    /// Construct concrete arguments from optional arguments
    fn from(args: OptionArgs) -> Self {
        Self {
            check: args.check.unwrap(),
            print: args.print.unwrap(),
            wrap: args.wrap.unwrap(),
            verbosity: args.verbosity.unwrap(),
            files: args.files,
            stdin: args.stdin.unwrap(),
            tabsize: args.tabsize.unwrap(),
            tabchar: args.tabchar.unwrap(),
            wraplen: args.wraplen.unwrap(),
            wrapmin: args.wrapmin.unwrap(),
            config: args.config,
        }
    }

    /// Resolve conflicting arguments
    pub fn resolve(&mut self, logs: &mut Vec<Log>) -> u8 {
        let mut exit_code = 0;
        self.print |= self.stdin;
        self.wrapmin = if self.wraplen >= 50 {
            self.wraplen - 10
        } else {
            self.wraplen
        };

        if !self.stdin && self.files.is_empty() {
            record_file_log(
                logs,
                Level::Error,
                "",
                "No files specified. Provide filenames or pass --stdin.",
            );
            exit_code = 1;
        }
        if self.stdin && !self.files.is_empty() {
            record_file_log(
                logs,
                Level::Error,
                "",
                "Do not provide file name(s) when using --stdin.",
            );
            exit_code = 1;
        }
        exit_code
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::from(OptionArgs::default())
    }
}
