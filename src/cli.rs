//! Functionality to parse CLI arguments

use crate::args::*;
use clap_complete::{generate, Shell};
use clap_mangen::Man;
use log::LevelFilter;
use std::io;

// Get the clap CLI command from a separate file
include!("command.rs");

/// Parse CLI arguments into `OptionArgs` struct
pub fn get_cli_args() -> OptionArgs {
    let mut command = get_cli_command();
    let arg_matches = command.clone().get_matches();

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
        arguments: if arg_matches.get_flag("args") {
            Some(true)
        } else {
            None
        },
        noconfig: if arg_matches.get_flag("noconfig") {
            Some(true)
        } else {
            None
        },
    };
    args
}
