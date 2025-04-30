//! Functionality to parse CLI arguments

use crate::args::*;
use clap::ArgMatches;
use clap_complete::{generate, Shell};
use clap_mangen::Man;
use log::LevelFilter;
use std::io;

// Get the clap CLI command from a separate file
include!("command.rs");

/// Read `ArgMatches` flag into `Option<bool>`
fn get_flag(arg_matches: &ArgMatches, flag: &str) -> Option<bool> {
    if arg_matches.get_flag(flag) {
        Some(true)
    } else {
        None
    }
}

/// Parse CLI arguments into `OptionArgs` struct
pub fn get_cli_args(matches: Option<ArgMatches>) -> OptionArgs {
    let mut command = get_cli_command();
    let arg_matches = match matches {
        Some(m) => m,
        None => command.clone().get_matches(),
    };

    // Generate completions and exit
    if let Some(shell) = arg_matches.get_one::<Shell>("completion") {
        generate(*shell, &mut command, "tex-fmt", &mut io::stdout());
        std::process::exit(0);
    }

    // Generate man page and exit
    if arg_matches.get_flag("man") {
        let man = Man::new(command);
        man.render(&mut io::stdout()).unwrap();
        std::process::exit(0);
    }

    let wrap: Option<bool> = if arg_matches.get_flag("nowrap") {
        Some(false)
    } else {
        None
    };
    let tabchar = if arg_matches.get_flag("usetabs") {
        Some(TabChar::Tab)
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
    let args = OptionArgs {
        check: get_flag(&arg_matches, "check"),
        print: get_flag(&arg_matches, "print"),
        fail_on_change: get_flag(&arg_matches, "fail-on-change"),
        wrap,
        wraplen: arg_matches.get_one::<u8>("wraplen").copied(),
        wrapmin: None,
        tabsize: arg_matches.get_one::<u8>("tabsize").copied(),
        tabchar,
        stdin: get_flag(&arg_matches, "stdin"),
        config: arg_matches.get_one::<PathBuf>("config").cloned(),
        noconfig: get_flag(&arg_matches, "noconfig"),
        lists: vec![],
        verbosity,
        arguments: get_flag(&arg_matches, "args"),
        files: arg_matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(ToOwned::to_owned)
            .collect::<Vec<String>>(),
    };
    args
}
