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

mod cli;
mod comments;
mod config;
mod format;
mod ignore;
mod indent;
mod logging;
mod read;
mod regexes;
mod subs;
mod verbatim;
mod wrap;
mod write;
use crate::cli::*;
use crate::config::*;
use crate::format::*;
use crate::logging::*;
use crate::read::*;
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
    let default_options = Config::default();
    // set args to cli. Will be overwritten if config file is found
    let mut config_file_args = default_options.clone();

    // check for config file and override defaults if found
    // TODO: find names for qualifier and organization
    if let Some(project_dirs) = ProjectDirs::from("", "", "tex-fmt") {
        let config_file = project_dirs.config_dir().join("config.toml");
        // TODO: also check for local config
        if config_file.exists() {
            config_file_args = Figment::new()
                .merge(Serialized::defaults(default_options))
                .merge(Toml::file(config_file))
                // TODO: override with local config if found
                .extract()
                .unwrap();
        }
    }
    let mut args = Config {
        check: cli_args.check.unwrap_or(config_file_args.check),
        print: cli_args.print.unwrap_or(config_file_args.print),
        keep: cli_args.keep.unwrap_or(config_file_args.keep),
        verbose: cli_args.verbose.unwrap_or(config_file_args.verbose),
        quiet: cli_args.quiet.unwrap_or(config_file_args.quiet),
        trace: cli_args.trace.unwrap_or(config_file_args.trace),
        files: cli_args.files.unwrap_or(config_file_args.files),
        stdin: cli_args.stdin.unwrap_or(config_file_args.stdin),
        tab: cli_args.tab.unwrap_or(config_file_args.tab),
        usetabs: cli_args.usetabs.unwrap_or(config_file_args.usetabs),
        wrap: cli_args.wrap.unwrap_or(config_file_args.wrap),
        wrap_min: cli_args.wrap_min.unwrap_or(config_file_args.wrap_min),
    };

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
