use crate::args::*;
use ArgAction::{Append, SetTrue};
use clap::{Arg, ArgAction, command, ArgMatches, value_parser};
use std::path::PathBuf;
use log::LevelFilter;

// read from cli to clap argmatches
fn get_arg_matches() -> ArgMatches {
    command!()
        // TODO help items
        .arg(Arg::new("check").short('c').long("check").action(SetTrue))
        .arg(Arg::new("print").short('p').long("print").action(SetTrue))
        .arg(Arg::new("wrap").short('w').long("wrap").action(SetTrue))
        .arg(Arg::new("nowrap").long("nowrap").action(SetTrue))
        .arg(Arg::new("verbose").short('v').long("verbose").action(SetTrue))
        .arg(Arg::new("quiet").short('q').long("quiet").action(SetTrue))
        .arg(Arg::new("trace").short('t').long("trace").action(SetTrue))
        .arg(Arg::new("files").action(Append))
        .arg(Arg::new("stdin").short('s').long("stdin").action(SetTrue))
        .arg(Arg::new("tabsize").short('t').long("tabsize")
             .value_parser(value_parser!(u8)).help("Tab size"))
        .arg(Arg::new("usetabs").long("usetabs").action(SetTrue))
        .arg(Arg::new("wraplen").long("wraplen")
             .value_parser(value_parser!(u8)).help("Wrap length"))
        .arg(Arg::new("wrapmin").long("wrapmin")
             .value_parser(value_parser!(u8)).help("Minimum wrap length"))
        .arg(Arg::new("config").long("config").help("Configuration file path")
            .value_parser(value_parser!(PathBuf)))
        .get_matches()
}

fn bool_to_option(b: bool) -> Option<bool> {
    if b {
        Some(true)
    } else {
        None
    }
}

fn flag_to_option(arg_matches: &ArgMatches, arg: &str) -> Option<bool> {
    bool_to_option(arg_matches.get_flag(arg))
}

// convert clap argmatches to args
pub fn get_cli_args() -> OptionArgs {
    let arg_matches = get_arg_matches();
    let wrap: Option<bool> = if arg_matches.get_flag("nowrap") {
        Some(false)
    } else if arg_matches.get_flag("wrap") {
        Some(true)
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
        check: flag_to_option(&arg_matches, "check"),
        print: flag_to_option(&arg_matches, "print"),
        wrap,
        verbosity,
        files: arg_matches.get_many::<String>("files")
            .unwrap_or_default().map(|v| v.to_owned()).collect::<Vec<String>>(),
        stdin: flag_to_option(&arg_matches, "stdin"),
        tabsize: arg_matches.get_one::<u8>("tabsize").copied(),
        tabchar,
        wraplen: arg_matches.get_one::<u8>("wraplen").copied(),
        wrapmin: arg_matches.get_one::<u8>("wrapmin").copied(),
        config: arg_matches.get_one::<PathBuf>("config").cloned(),
    };
    args
}
