use clap::Parser;
use log::Level::Error;
use std::fs;
use std::process::exit;

mod colors;
mod comments;
mod format;
mod ignore;
mod indent;
mod leave;
mod logging;
mod parse;
mod regexes;
mod subs;
mod wrap;
mod write;
use crate::format::*;
use crate::logging::*;
use crate::parse::*;
use crate::write::*;

const TAB: i8 = 2;

#[cfg(test)]
mod tests;

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
                record_file_log(
                    &mut logs,
                    Error,
                    file,
                    "Could not open file.",
                );
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
