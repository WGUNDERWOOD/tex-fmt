//! Utilities for reading the command line arguments

use crate::{logging::*, Config};
use clap::Parser;
use log::Level::Error;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;

/// Command line arguments
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Parser, Serialize, Deserialize, Clone)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, short, help = "Check formatting, do not modify files")]
    pub check: Option<bool>,
    #[arg(long, short, help = "Print to STDOUT, do not modify files")]
    pub print: Option<bool>,
    #[arg(long, short, help = "Keep lines, do not wrap")]
    pub keep: Option<bool>,
    #[arg(long, short, help = "Show info log messages")]
    pub verbose: Option<bool>,
    #[arg(long, short, help = "Hide warning messages")]
    pub quiet: Option<bool>,
    #[arg(long, short, help = "Show trace log messages")]
    pub trace: Option<bool>,
    #[arg(help = "List of files to be formatted")]
    pub files: Option<Vec<String>>,
    #[arg(
        long,
        short,
        help = "Process STDIN as a single file, output formatted text to STDOUT"
    )]
    pub stdin: Option<bool>,
    #[arg(long, help = "Number of spaces to use as tab size")]
    pub tab: Option<i8>,
    #[arg(long, help = "Use tabs instead of spaces for indentation")]
    pub usetabs: Option<bool>,
    #[arg(long, help = "Line length for wrapping")]
    pub wrap: Option<u8>,
    #[clap(skip)]
    pub wrap_min: Option<u8>,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            check: Some(false),
            print: Some(false),
            keep: Some(false),
            verbose: Some(false),
            quiet: Some(false),
            trace: Some(false),
            files: Some(vec![]),
            stdin: Some(false),
            tab: Some(2),
            usetabs: Some(false),
            wrap: Some(80),
            wrap_min: Some(70),
        }
    }
}

impl Into<Config> for Cli {
    fn into(self) -> Config {
        let mut config = Config::default();
        config.check = self.check.unwrap_or(false);
        config.print = self.print.unwrap_or(false);
        config.keep = self.keep.unwrap_or(false);
        config.verbose = self.verbose.unwrap_or(false);
        config.quiet = self.quiet.unwrap_or(false);
        config.trace = self.trace.unwrap_or(false);
        config.files = self.files.unwrap_or(vec![]);
        config.stdin = self.stdin.unwrap_or(false);
        config.tab = self.tab.unwrap_or(2);
        config.usetabs = self.usetabs.unwrap_or(false);
        config.wrap = self.wrap.unwrap_or(80);
        config.wrap_min = self.wrap_min.unwrap_or(70);
        config
    }
}
