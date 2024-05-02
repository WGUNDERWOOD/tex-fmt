use clap::Parser;
use std::env::temp_dir;
use std::fs;
use std::path;

const TAB: i8 = 2;

const YELLOW: &str = "\x1b[33m\x1b[1m";
const PINK: &str = "\x1b[35m\x1b[1m";
const RESET: &str = "\x1b[00m\x1b[0m";

#[derive(Parser)]
struct Cli {
    #[arg(long, short, help = "Print to stdout, do not modify files")]
    print: bool,
    #[arg(
        long,
        short,
        help = "Debug mode, disable checks and do not modify files"
    )]
    debug: bool,
    #[arg(required = true)]
    filenames: Vec<String>,
}

mod format;
mod indent;
mod regexes;
mod subs;
mod comments;
use crate::format::*;

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

    // print script name
    println!("{}", String::new() + PINK + "tex-fmt" + RESET);

    for filename in filenames {
        // print file name
        if debug {
            println!("{}", String::new() + YELLOW + &filename + RESET);
        }

        // read lines from file
        let file =
            fs::read_to_string(&filename).expect("Should have read the file");

        let new_file = format_file(&file, debug);

        if print {
            // print new file
            println!("{}", &new_file);
        } else {
            // backup original file
            let filepath = path::Path::new(&filename).canonicalize().unwrap();
            let mut filebak = temp_dir();
            filebak.push("tex-fmt");
            fs::create_dir_all(&filebak).unwrap();
            filebak.push(filepath.file_name().unwrap());
            fs::copy(filepath.clone(), &filebak).unwrap();

            // write new file
            fs::write(filepath, new_file).unwrap();
        }
    }
}
