use clap::Parser;
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
    let args = Cli::parse();
    let debug = args.debug;
    let mut print = args.print;
    let filenames = args.filenames;
    if debug {
        print = true;
    };

    // check files are in correct format
    assert!(filenames.iter().all(|f| f.ends_with(".tex")
        || f.ends_with(".bib")
        || f.ends_with(".sty")
        || f.ends_with(".cls")));

    print_script_name();

    for filename in filenames {
        if debug {
            print_file_name(&filename);
        }

        // read lines from file
        let file =
            fs::read_to_string(&filename).expect("Should have read the file");

        let new_file = format_file(&file, debug);

        if print {
            print_file(&new_file);
        } else {
            backup_file(&filename);
            write_file(&filename, &new_file);
        }
    }
}
