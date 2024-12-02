//! Functionality to parse CLI arguments

use crate::args::*;
use clap::{command, Command, value_parser, Arg, ArgAction};
use log::LevelFilter;
use std::borrow::ToOwned;
use std::path::PathBuf;
use ArgAction::{Append, SetTrue};

/// Construct the CLI command
fn get_cli_command() -> Command {
    command!()
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .action(SetTrue)
                .help("Check formatting, do not modify files"),
        )
        .arg(
            Arg::new("print")
                .short('p')
                .long("print")
                .action(SetTrue)
                .help("Print to stdout, do not modify files"),
        )
        .arg(
            Arg::new("nowrap")
                .short('n')
                .long("nowrap")
                .action(SetTrue)
                .help("Do not wrap long lines"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(SetTrue)
                .help("Show info messages"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(SetTrue)
                .help("Hide warning messages"),
        )
        .arg(
            Arg::new("trace")
                .long("trace")
                .action(SetTrue)
                .help("Show trace messages"),
        )
        .arg(
            Arg::new("files")
                .action(Append)
                .help("List of files to be formatted"),
        )
        .arg(
            Arg::new("stdin")
                .short('s')
                .long("stdin")
                .action(SetTrue)
                .help("Process stdin as a single file, output to stdout"),
        )
        .arg(
            Arg::new("tabsize")
                .short('t')
                .long("tabsize")
                .value_parser(value_parser!(u8))
                .help("Number of characters to use as tab size [default: 2]"),
        )
        .arg(
            Arg::new("usetabs")
                .long("usetabs")
                .action(SetTrue)
                .help("Use tabs instead of spaces for indentation"),
        )
        .arg(
            Arg::new("wraplen")
                .short('l')
                .long("wraplen")
                .value_parser(value_parser!(u8))
                .help("Line length for wrapping [default: 80]"),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .help("Configuration file path")
                .value_parser(value_parser!(PathBuf))
                .help("Path to config file"),
        )
}

/// Parse CLI arguments into `OptionArgs` struct
pub fn get_cli_args() -> OptionArgs {
    let arg_matches = get_cli_command().get_matches();
    let wrap: Option<bool> = if arg_matches.get_flag("nowrap") {
        Some(false)
    } else {
        None
    };
    let verbosity = if arg_matches.get_flag("trace") {
        Some(LevelFilter::Trace)
    } else if arg_matches.get_flag("verbose") {
        Some(LevelFilter::Info)
    } else if arg_matches.get_flag("quiet") {
        Some(LevelFilter::Error)
    } else {
        None
    };
    let tabchar = if arg_matches.get_flag("usetabs") {
        Some(TabChar::Tab)
    } else {
        None
    };
    let args = OptionArgs {
        check: if arg_matches.get_flag("check") {
            Some(true)
        } else {
            None
        },
        print: if arg_matches.get_flag("print") {
            Some(true)
        } else {
            None
        },
        wrap,
        verbosity,
        files: arg_matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(ToOwned::to_owned)
            .collect::<Vec<String>>(),
        stdin: if arg_matches.get_flag("stdin") {
            Some(true)
        } else {
            None
        },
        tabsize: arg_matches.get_one::<u8>("tabsize").copied(),
        tabchar,
        wraplen: arg_matches.get_one::<u8>("wraplen").copied(),
        wrapmin: None,
        config: arg_matches.get_one::<PathBuf>("config").cloned(),
    };
    args
}
