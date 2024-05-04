use clap::Parser;
#[allow(unused_imports)]
use rstest::rstest;
#[allow(unused_imports)]
use rstest_reuse::{self, *};
use std::fs;

const TAB: i8 = 2;

mod comments;
mod format;
mod indent;
mod parse;
mod print;
mod regexes;
mod subs;
mod wrap;
mod write;
use crate::format::*;
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

    // check files are in correct format
    assert!(args.filenames.iter().all(|f| f.ends_with(".tex")
        || f.ends_with(".bib")
        || f.ends_with(".sty")
        || f.ends_with(".cls")));

    print_script_name();

    for filename in args.filenames {
        if args.debug {
            print_file_name(&filename);
        }

        // read lines from file
        let file =
            fs::read_to_string(&filename).expect("Should have read the file");

        let new_file = format_file(&file, args.debug);

        if args.print {
            print_file(&new_file);
        } else {
            backup_file(&filename);
            write_file(&filename, &new_file);
        }
    }
}
