//! tex-fmt
//! An extremely fast LaTeX formatter written in Rust

#![warn(missing_docs)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::module_name_repetitions)]

use clap::Parser;
use log::Level::Error;
use std::fs;
use std::process::exit;

mod colors;
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
    args.resolve();
    init_logger(&args);
    let mut exit_code = 0;

    for file in &args.files {
        let mut logs = Vec::<Log>::new();
        let extension_valid = check_extension_valid(file);
        if extension_valid {
            if let Ok(text) = fs::read_to_string(file) {
                let new_text = format_file(&text, file, &args, &mut logs);
                if args.print {
                    println!("{}", &new_text);
                } else if args.check && text != new_text {
                    record_file_log(
                        &mut logs,
                        Error,
                        file,
                        "Incorrect formatting.",
                    );
                    exit_code = 1;
                } else if text != new_text {
                    write_file(file, &new_text);
                }
            } else {
                record_file_log(&mut logs, Error, file, "Could not open file.");
                exit_code = 1;
            }
        } else {
            record_file_log(&mut logs, Error, file, "File type invalid.");
            exit_code = 1;
        };

        print_logs(logs);
    }
    exit(exit_code)
}
