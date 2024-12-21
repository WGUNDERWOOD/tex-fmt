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

use std::fs;
use std::process::ExitCode;

mod args;
mod cli;
mod comments;
mod config;
mod format;
mod ignore;
mod indent;
mod logging;
mod pattern;
mod read;
mod regexes;
mod subs;
mod verbatim;
mod wrap;
mod write;
use crate::args::*;
use crate::format::*;
use crate::logging::*;

#[cfg(test)]
mod tests;

#[cfg(target_family = "unix")]
/// Line ending for unix
const LINE_END: &str = "\n";

#[cfg(target_family = "windows")]
/// Line ending for Windows
const LINE_END: &str = "\r\n";

fn main() -> ExitCode {
    let mut args = get_args();
    init_logger(args.verbosity);

    let mut logs = Vec::<Log>::new();
    let mut exit_code = args.resolve(&mut logs);

    if exit_code == 0 {
        exit_code = run(&args, &mut logs);
    }

    print_logs(&mut logs);
    ExitCode::from(exit_code)
}
