//! tex-fmt-lib
//! An extremely fast LaTeX formatter written in Rust

#![warn(missing_docs)]

use clap::Parser;
use cli::Cli;
use format::*;
use logging::*;
use read::*;
use std::fs;
use write::*;

mod cli;
mod comments;
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

#[cfg(test)]
mod tests;

/// Default line ending
#[cfg(not(target_family = "windows"))]
const LINE_END: &str = "\n";

#[cfg(target_family = "windows")]
/// Line ending for Windows
const LINE_END: &str = "\r\n";

/// Executes the main program logic and returns an exit code.
pub fn run(input: Option<&str>, output: &mut Option<String>) -> u8 {
    let mut args = Cli::parse();
    init_logger(args.log_level());

    let mut logs = Vec::<Log>::new();

    if input.is_some() {
        args.stdin = true;
    }

    let mut exit_code = args.resolve(&mut logs);

    if exit_code == 0 {
        if let Some(text) = input {
            let file = String::from("<STDIN>");
            let new_text = format_file(&text, &file, &args, &mut logs);
            if let Some(ref mut output_text) = output {
                // Write the result to output
                output_text.push_str(&new_text);
                exit_code = 1
            } else {
                // Inspect the args to figure out what to do with the
                // result.
                exit_code = process_output(
                    &args, &file, &text, &new_text, exit_code, &mut logs,
                );
            }
        } else if args.stdin {
            // TODO combine the read and read_stdin functions to simplify this
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
    exit_code
}
