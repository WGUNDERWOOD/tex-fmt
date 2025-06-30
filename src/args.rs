//! Main arguments

use crate::cli::get_cli_args;
use crate::config::{get_config, get_config_args};
use crate::logging::{record_file_log, Log};
use crate::search::find_files;
use colored::Colorize;
use log::Level;
use log::LevelFilter;
use merge::Merge;
use std::fmt;
use std::path::PathBuf;

const DISPLAY_HEADER_WIDTH: usize = 24;

/// Arguments passed to tex-fmt
#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    /// Check formatting, do not modify files
    pub check: bool,
    /// Print to stdout, do not modify files
    pub print: bool,
    /// Return non-0 exit code when modifying files
    pub fail_on_change: bool,
    /// Wrap long lines
    pub wrap: bool,
    /// Maximum allowed line length
    pub wraplen: u8,
    /// Wrap lines longer than this
    pub wrapmin: u8,
    /// Number of characters to use as tab size
    pub tabsize: u8,
    /// Characters to use for indentation
    pub tabchar: TabChar,
    /// Read from stdin and output to stdout
    pub stdin: bool,
    /// Path to config file
    pub config: Option<PathBuf>,
    /// Extra list environments
    pub lists: Vec<String>,
    /// Extra verbatim environments
    pub verbatims: Vec<String>,
    /// Environments which are not indented
    pub no_indent_envs: Vec<String>,
    /// Characters after which lines can be wrapped
    pub wrap_chars: Vec<char>,
    /// Verbosity level for log messages
    pub verbosity: LevelFilter,
    /// Print arguments and exit
    pub arguments: bool,
    /// List of files to be formatted
    pub files: Vec<String>,
    /// Recursive search for files
    pub recursive: bool,
}

/// Arguments using Options to track CLI/config file/default values
#[derive(Clone, Debug, Merge)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct OptionArgs {
    pub check: Option<bool>,
    pub print: Option<bool>,
    pub fail_on_change: Option<bool>,
    pub wrap: Option<bool>,
    pub wraplen: Option<u8>,
    pub wrapmin: Option<u8>,
    pub tabsize: Option<u8>,
    pub tabchar: Option<TabChar>,
    pub stdin: Option<bool>,
    pub config: Option<PathBuf>,
    pub noconfig: Option<bool>,
    #[merge(strategy = merge::vec::append)]
    pub lists: Vec<String>,
    #[merge(strategy = merge::vec::append)]
    pub verbatims: Vec<String>,
    #[merge(strategy = merge::vec::append)]
    pub no_indent_envs: Vec<String>,
    #[merge(strategy = merge::vec::append)]
    pub wrap_chars: Vec<char>,
    pub verbosity: Option<LevelFilter>,
    pub arguments: Option<bool>,
    #[merge(strategy = merge::vec::append)]
    pub files: Vec<String>,
    pub recursive: Option<bool>,
}

/// Character to use for indentation
#[derive(Clone, Debug, PartialEq, Eq)]
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
        let lists = vec![
            "itemize",
            "enumerate",
            "description",
            "inlineroman",
            "inventory",
        ]
        .into_iter()
        .map(std::borrow::ToOwned::to_owned)
        .collect();
        let verbatims =
            vec!["verbatim", "Verbatim", "lstlisting", "minted", "comment"]
                .into_iter()
                .map(std::borrow::ToOwned::to_owned)
                .collect();
        let no_indent_envs = vec!["document"]
            .into_iter()
            .map(std::borrow::ToOwned::to_owned)
            .collect();
        let wrap_chars = vec![' '];
        Self {
            check: Some(false),
            print: Some(false),
            fail_on_change: Some(false),
            wrap: Some(true),
            wraplen: Some(80),
            wrapmin: Some(70),
            tabsize: Some(2),
            tabchar: Some(TabChar::Space),
            stdin: Some(false),
            config: None,
            noconfig: Some(false),
            lists,
            verbatims,
            no_indent_envs,
            wrap_chars,
            verbosity: Some(LevelFilter::Warn),
            arguments: Some(false),
            files: vec![],
            recursive: Some(false),
        }
    }
}

impl OptionArgs {
    #[must_use]
    pub fn new() -> Self {
        Self {
            check: None,
            print: None,
            fail_on_change: None,
            wrap: None,
            wraplen: None,
            wrapmin: None,
            tabsize: None,
            tabchar: None,
            stdin: None,
            config: None,
            noconfig: None,
            lists: vec![],
            verbatims: vec![],
            no_indent_envs: vec![],
            wrap_chars: vec![],
            verbosity: None,
            arguments: None,
            files: vec![],
            recursive: None,
        }
    }
}

/// Get all arguments from CLI, config file, and defaults, and merge them
#[must_use]
pub fn get_args() -> Args {
    let mut args: OptionArgs = get_cli_args(None);
    let config = get_config(&args);
    let config_args: Option<OptionArgs> = get_config_args(config);
    if let Some(c) = config_args {
        args.merge(c);
    }
    args.merge(OptionArgs::default());
    Args::from(args)
}

impl Args {
    /// Construct concrete arguments from optional arguments
    ///
    /// # Panics
    ///
    /// This function panics when called on `OptionArgs` with `None` fields.
    /// However this should never happen as merging in `OptionArgs::default()`
    /// should overwrite any `None` fields.
    #[must_use]
    pub fn from(args: OptionArgs) -> Self {
        Self {
            check: args.check.unwrap(),
            print: args.print.unwrap(),
            fail_on_change: args.fail_on_change.unwrap(),
            wrap: args.wrap.unwrap(),
            wraplen: args.wraplen.unwrap(),
            wrapmin: args.wrapmin.unwrap(),
            tabsize: args.tabsize.unwrap(),
            tabchar: args.tabchar.unwrap(),
            stdin: args.stdin.unwrap(),
            config: args.config,
            lists: args.lists,
            verbatims: args.verbatims,
            no_indent_envs: args.no_indent_envs,
            wrap_chars: args.wrap_chars,
            verbosity: args.verbosity.unwrap(),
            arguments: args.arguments.unwrap(),
            files: args.files,
            recursive: args.recursive.unwrap(),
        }
    }

    /// Resolve conflicting arguments
    pub fn resolve(&mut self, logs: &mut Vec<Log>) -> u8 {
        let mut exit_code = 0;

        // stdin implies print
        self.print |= self.stdin;

        // Set wrapmin
        self.wrapmin = if self.wraplen >= 50 {
            self.wraplen - 10
        } else {
            self.wraplen
        };

        // Recursive file search
        if self.recursive {
            let tmp = if self.files.is_empty() {
                vec!["./".to_string()]
            } else {
                self.files.clone()
            };

            for dir in &tmp {
                find_files(dir.into(), &mut self.files);
            }

            self.files.retain(|e| PathBuf::from(e).is_file());
        }

        // Check if directory is passed without --recursive
        if !self.recursive
            && self.files.iter().any(|e| PathBuf::from(e).is_dir())
        {
            record_file_log(
                logs,
                Level::Error,
                "",
                "A directory was passed without --recursive.",
            );
            exit_code = 1;
        }

        // Check files are passed if no --stdin or --recursive
        if !self.stdin && self.files.is_empty() {
            record_file_log(
                logs,
                Level::Error,
                "",
                "No files specified. Provide filenames, or pass --recursive or --stdin.",
            );
            exit_code = 1;
        }

        // Check no files are passed if --stdin
        if self.stdin && !self.files.is_empty() {
            record_file_log(
                logs,
                Level::Error,
                "",
                "Do not provide file name(s) when using --stdin.",
            );
            exit_code = 1;
        }

        // Remove duplicate environments and files
        self.lists.dedup();
        self.verbatims.dedup();
        self.no_indent_envs.dedup();
        self.wrap_chars.dedup();
        self.files.dedup();

        // Print arguments and exit
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
    let width = DISPLAY_HEADER_WIDTH;
    let name_fmt = format!("{}{}", name.bold(), ":");
    write!(f, "\n  {name_fmt:<width$} {value}")?;
    Ok(())
}

/// Display an argument field which is a list of strings
fn display_args_list(
    v: &[String],
    name: &str,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    if !v.is_empty() {
        display_arg_line(f, name, &v[0])?;
        for x in &v[1..] {
            write!(
                f,
                "\n  {:<width$} {}",
                "".bold().to_string(),
                x,
                width = DISPLAY_HEADER_WIDTH
            )?;
        }
    }
    Ok(())
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let wrap_chars: Vec<String> = self
            .wrap_chars
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        write!(f, "{}", "tex-fmt".magenta().bold())?;
        display_arg_line(f, "check", &self.check.to_string())?;
        display_arg_line(f, "print", &self.print.to_string())?;
        display_arg_line(f, "fail-on-change", &self.print.to_string())?;
        display_arg_line(f, "wrap", &self.wrap.to_string())?;
        display_arg_line(f, "wraplen", &self.wraplen.to_string())?;
        display_arg_line(f, "wrapmin", &self.wrapmin.to_string())?;
        display_arg_line(f, "tabsize", &self.tabsize.to_string())?;
        display_arg_line(f, "tabchar", &self.tabchar.to_string())?;
        display_arg_line(f, "stdin", &self.stdin.to_string())?;
        match &self.config {
            None => display_arg_line(f, "config", "None")?,
            Some(c) => display_arg_line(f, "config", &c.display().to_string())?,
        }
        display_arg_line(
            f,
            "verbosity",
            &self.verbosity.to_string().to_lowercase(),
        )?;
        display_args_list(&self.lists, "lists", f)?;
        display_args_list(&self.verbatims, "verbatims", f)?;
        display_args_list(&self.no_indent_envs, "no-indent-envs", f)?;
        display_args_list(&wrap_chars, "wrap-chars", f)?;
        display_args_list(&self.files, "files", f)?;

        // Do not print `arguments` or `noconfig` fields
        Ok(())
    }
}
