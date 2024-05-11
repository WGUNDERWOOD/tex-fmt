use clap::Parser;
use env_logger::Builder;
#[allow(unused_imports)]
use rstest::rstest;
#[allow(unused_imports)]
use rstest_reuse::{self, *};
use std::fs;

const TAB: i8 = 2;

mod comments;
mod format;
mod indent;
mod logging;
mod parse;
mod print;
mod regexes;
mod subs;
mod wrap;
mod write;
use crate::format::*;
use crate::logging::*;
use crate::parse::*;
use crate::print::*;
use crate::write::*;

#[cfg(test)]
mod tests;

fn main() {
    // get arguments
    let mut args = Cli::parse();
    if args.debug {
        args.print = true;
        args.verbose = true;
    };

    // initialize logger
    init_logger(&args);

    // check files are in correct format
    let extensions = [".tex", ".bib", ".sty", ".cls"];
    for f in &args.filenames {
        let mut extension_valid = false;
        for extension in extensions {
            if f.ends_with(extension) {
                extension_valid = true;
            }
        }
        if !extension_valid {
            log::error!("File type invalid for {}", f);
            panic!();
        }
    }

    print_script_name();

    for filename in &args.filenames {
        if args.debug {
            print_file_name(filename);
        }

        // read lines from file
        let file =
            fs::read_to_string(filename).expect("Should have read the file");

        let new_file = format_file(&file, &args);

        if args.print {
            print_file(&new_file);
        } else {
            backup_file(filename);
            write_file(filename, &new_file);
        }
    }
}
