use clap::Parser;
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
mod print;
mod regexes;
mod subs;
mod wrap;
mod write;
use crate::colors::*;
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

    init_logger(&args);
    print_script_name();

    for filename in &args.filenames {
        if args.verbose {
            print_file_name(filename);
        }

        if !check_extension_valid(filename) {
            log::error!("File type invalid for {}{}", WHITE, filename);
            continue;
        };

        let file = fs::read_to_string(filename).unwrap();
        let new_file = format_file(&file, &args);

        if args.print {
            print_file(&new_file);
        } else {
            write_file(filename, &new_file);
        }
    }
}
