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
use std::fs;
use std::process::exit;

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

fn main() {
    let mut args = Cli::parse();
    init_logger(args.log_level());

    args.resolve();
    let mut exit_code = 0;

    if args.stdin {
        let mut logs = vec![];
        if let Some((file, text)) = read_stdin(&mut logs) {
            let new_text = format_file(&text, &file, &args, &mut logs);
            exit_code = process_output(
                &args, &file, &text, &new_text, exit_code, &mut logs,
            );
        } else {
            exit_code = 1;
        }
        print_logs(logs);
    } else {
        for file in &args.files {
            let mut logs = vec![];
            if let Some((file, text)) = read(file, &mut logs) {
                let new_text = format_file(&text, &file, &args, &mut logs);
                exit_code = process_output(
                    &args, &file, &text, &new_text, exit_code, &mut logs,
                );
            } else {
                exit_code = 1;
            };
            print_logs(logs);
        }
    }

    exit(exit_code)
}
