//! tex-fmt
//! An extremely fast LaTeX formatter written in Rust

#![warn(missing_docs)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::module_name_repetitions)]

use clap::Parser;
use directories::ProjectDirs;
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use std::fs;
use std::process::ExitCode;

mod comments;
mod format;
mod ignore;
mod indent;
mod logging;
mod parse;
mod regexes;
mod subs;
mod verbatim;
mod wrap;
mod write;
use crate::format::*;
use crate::logging::*;
use crate::parse::*;
use crate::write::*;

#[cfg(test)]
mod tests;

#[cfg(target_family = "unix")]
/// Line ending for unix
const LINE_END: &str = "\n";

#[cfg(target_family = "windows")]
/// Line ending for Windows
const LINE_END: &str = "\r\n";

fn main() -> ExitCode {
    // parse CLI arguments
    let cli_args = Cli::parse();

    // initilize default config
    let default_options = Cli {
        check: false,
        print: false,
        keep: false,
        verbose: false,
        stdin: false,
        quiet: false,
        trace: false,
        files: Vec::<String>::new(),
        tab: 2,
        usetabs: false,
        wrap: 80,
        wrap_min: 70,
    };
    // set args to cli. Will be overwritten if config file is found
    let mut args = cli_args.clone();

    // check for config file and override defaults if found
    // TODO: find names for qualifier and organization
    if let Some(project_dirs) = ProjectDirs::from("", "", "tex-fmt") {
        let config_file = project_dirs.config_dir().join("config.toml");
        // TODO: also check for local config
        if config_file.exists() {
            args = Figment::new()
                .merge(Serialized::defaults(default_options.clone()))
                .merge(Toml::file(config_file))
                // TODO: override with local config if found
                .extract()
                .unwrap();

            if cli_args.check != default_options.check {
                args.check = cli_args.check;
            }
            if cli_args.print != default_options.print {
                args.print = cli_args.print;
            }
            // NOTE: If keep is set to true in config file, it can't be set to false with the CLI
            if cli_args.keep != default_options.keep {
                args.keep = cli_args.keep;
            }
            if cli_args.verbose != default_options.verbose {
                args.verbose = cli_args.verbose;
            }
            if cli_args.stdin != default_options.stdin {
                args.stdin = cli_args.stdin;
            }
            if cli_args.quiet != default_options.quiet {
                args.quiet = cli_args.quiet;
            }
            if cli_args.trace != default_options.trace {
                args.trace = cli_args.trace;
            }
            if cli_args.files != default_options.files {
                args.files = cli_args.files;
            }
            if cli_args.tab != default_options.tab {
                args.tab = cli_args.tab;
            }
            if cli_args.usetabs != default_options.usetabs {
                args.usetabs = cli_args.usetabs;
            }
            if cli_args.wrap != default_options.wrap {
                args.wrap = cli_args.wrap;
            }
            if cli_args.wrap_min != default_options.wrap_min {
                args.wrap_min = cli_args.wrap_min;
            }
        }
    }

    init_logger(args.log_level());

    let mut logs = Vec::<Log>::new();
    let mut exit_code = args.resolve(&mut logs);

    if exit_code == 0 {
        if args.stdin {
            if let Some((file, text)) = read_stdin(&mut logs) {
                let new_text = format_file(&text, &file, &args, &mut logs);
                exit_code = process_output(
                    &args, &file, &text, &new_text, exit_code, &mut logs,
                );
            } else {
                exit_code = 1;
            }
        } else {
            for file in &args.files {
                if let Some((file, text)) = read(file, &mut logs) {
                    let new_text = format_file(&text, &file, &args, &mut logs);
                    exit_code = process_output(
                        &args, &file, &text, &new_text, exit_code, &mut logs,
                    );
                } else {
                    exit_code = 1;
                };
            }
        }
    }

    print_logs(&mut logs);
    ExitCode::from(exit_code)
}
