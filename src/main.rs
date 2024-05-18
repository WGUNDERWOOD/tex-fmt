use clap::Parser;
use log::Level::{Error, Info};
#[allow(unused_imports)]
use rstest::rstest;
#[allow(unused_imports)]
use rstest_reuse::{self, *};
use std::fs;

const TAB: i8 = 2;

mod colors;
mod comments;
mod format;
mod ignore;
mod indent;
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

#[cfg(test)]
mod tests;

fn main() {
    let mut args = Cli::parse();
    if args.debug {
        args.print = true;
        args.verbose = true;
    };

    init_logger(&args);

    for filename in &args.filenames {
        let mut logs: Vec<Log> = vec![];
        if args.verbose {
            record_log(
                &mut logs,
                Info,
                None,
                filename.to_string(),
                None,
                None,
                "Begin indenting.".to_string(),
            );
        }

        let extension_valid = check_extension_valid(filename);
        if extension_valid {
            let file = fs::read_to_string(filename).unwrap();
            let new_file = format_file(&file, filename, &args, &mut logs);
            if args.print {
                println!("{}", &new_file);
            } else {
                write_file(filename, &new_file);
            }
        } else {
            record_log(
                &mut logs,
                Error,
                None,
                filename.to_string(),
                None,
                None,
                "File type invalid.".to_string(),
            );
        };

        print_logs(&args, logs);
    }
}
