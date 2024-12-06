//! Main arguments

use crate::cli::*;
use crate::config::*;
use crate::logging::*;
use crate::Log;
use colored::Colorize;
use log::Level;
use log::LevelFilter;
use merge::Merge;
use std::fmt;
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
    /// Print arguments and exit
    pub arguments: bool,
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
    pub arguments: Option<bool>,
}

/// Character to use for indentation
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum TabChar {
    Tab,
    Space,
}

impl fmt::Display for TabChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Tab => write!(f, "tab"),
            Self::Space => write!(f, "space"),
        }
    }
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
            arguments: Some(false),
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
            arguments: args.arguments.unwrap(),
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
        if self.arguments {
            println!("{self}");
            std::process::exit(0);
        }
        exit_code
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::from(OptionArgs::default())
    }
}

/// Print a field from `Args`
fn display_arg_line(
    f: &mut fmt::Formatter,
    name: &str,
    value: &str,
) -> fmt::Result {
    let width = 20;
    let name_fmt = format!("{}{}", name.bold(), ":");
    write!(f, "\n  {name_fmt:<width$} {value}")?;
    Ok(())
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "tex-fmt".magenta().bold())?;
        display_arg_line(f, "check", &self.check.to_string())?;
        display_arg_line(f, "print", &self.print.to_string())?;
        display_arg_line(f, "wrap", &self.wrap.to_string())?;
        display_arg_line(
            f,
            "verbosity",
            &self.verbosity.to_string().to_lowercase(),
        )?;

        if !self.files.is_empty() {
            display_arg_line(f, "files", &self.files[0])?;
            for file in &self.files[1..] {
                write!(
                    f,
                    "\n  {:<width$} {}",
                    "".bold().to_string(),
                    file,
                    width = 20
                )?;
            }
        }

        display_arg_line(f, "stdin", &self.stdin.to_string())?;
        display_arg_line(f, "tabsize", &self.tabsize.to_string())?;
        display_arg_line(f, "tabchar", &self.tabchar.to_string())?;
        display_arg_line(f, "wraplen", &self.wraplen.to_string())?;
        display_arg_line(f, "wrapmin", &self.wrapmin.to_string())?;
        match &self.config {
            None => display_arg_line(f, "config", "None")?,
            Some(c) => display_arg_line(f, "config", &c.display().to_string())?,
        }
        // Note we do not print the `arguments` field as it's not useful
        Ok(())
    }
}
