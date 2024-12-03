//! Functionality to parse CLI arguments

use crate::args::*;
use log::LevelFilter;

// Get the clap CLI command from a separate file
include!("command.rs");

/// Parse CLI arguments into `OptionArgs` struct
pub fn get_cli_args() -> OptionArgs {
    let arg_matches = get_cli_command().get_matches();
    get_completions(&arg_matches);
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

fn print_completions<G: Generator>(gen: G, command: &mut Command) {
    generate(gen, command, command.get_name().to_string(), &mut io::stdout());
}

fn get_completions(arg_matches: &ArgMatches) {

    if let Some(generator) = arg_matches.get_one::<Shell>("generator") {
        let mut command = get_cli_command();
        eprintln!("Generating completion file for {generator}...");
        print_completions(*generator, &mut command);
    }
}
