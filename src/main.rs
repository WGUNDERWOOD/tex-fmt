use clap::Parser;
//use lazy_static::lazy_static;
//use regex::Regex;
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

pub mod regexes;
use crate::regexes::regexes::*;

fn remove_extra_newlines(file: &str) -> String {
    RE_NEWLINES.replace_all(file, "\n\n").to_string()
}

fn remove_tabs(file: &str) -> String {
    let replace = (0..TAB).map(|_| " ").collect::<String>();
    RE_TABS.replace_all(file, replace).to_string()
}

fn remove_trailing_spaces(file: &str) -> String {
    RE_TRAIL.replace_all(file, "\n").to_string()
}

fn remove_comment(line: &str) -> String {
    let new_line = RE_PERCENT.replace_all(line, "").to_string();
    RE_COMMENT.replace_all(&new_line, "").to_string()
}

mod indent;
use crate::indent::indent::*;

fn format_file(file: String, debug: bool) -> String {
    // preformat
    let mut new_file = remove_extra_newlines(&file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    let lines: Vec<&str> = new_file.lines().collect();

    // set up variables
    //let mut count: i8 = 0;
    let n_lines = lines.len();
    let mut indent = Indent{actual: 0, visual: 0, item: 0};
    let mut new_lines = vec!["".to_owned(); n_lines];

    // main loop through file
    for i in 0..n_lines {
        // calculate indent
        let line = lines[i];
        let line_strip = &remove_comment(line);
        indent = get_indent(line_strip, indent);
        //let back = get_back(line_strip);
        //let diff = get_diff(line_strip);
        //let indent: i8 = count - back;
        if !debug {
            dbg!(&line);
            dbg!(&indent.actual);
            dbg!(&indent.visual);
            dbg!();
            //assert!(indent.actual >= 0, "line {}", i);
            //assert!(indent.visual >= 0, "line {}", i);
        };
        //indents[i] = indent;
        //count += diff;

        // apply indent
        let mut new_line = line.trim_start().to_string();
        if !new_line.is_empty() {
            let n_spaces = indent.visual * TAB;
            let spaces: String = (0..n_spaces).map(|_| " ").collect();
            new_line.insert_str(0, &spaces);
        }
        new_lines[i] = new_line
    }

    // check indents return to zero
    //if !debug {
        //assert!(indent.current == &0);
    //}

    // prepare indented file
    let mut new_file = new_lines.join("\n");
    new_file.push('\n');
    new_file
}

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

        let new_file = format_file(file, debug);

        if print {
            // print new file
            println!("{}", &new_file);
        } else {
            // backup original file
            let filepath = path::Path::new(&filename).canonicalize().unwrap();
            let mut fileback = temp_dir();
            fileback.push("tex-fmt");
            fs::create_dir_all(&fileback).unwrap();
            fileback.push(filepath.file_name().unwrap());
            fs::copy(filepath.clone(), &fileback).unwrap();

            // write new file
            fs::write(filepath, new_file).unwrap();
        }
    }
}
