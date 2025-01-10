//! Utilities for writing formatted files

use crate::args::*;
use crate::logging::*;
use log::Level::Error;
use std::fs;
use std::path;

/// Write a formatted file to disk
fn write_file(file: &str, text: &str) {
    let filepath = path::Path::new(&file).canonicalize().unwrap();
    fs::write(filepath, text).expect("Could not write the file");
}

/// Handle the newly formatted file
pub fn process_output(
    args: &Args,
    file: &str,
    text: &str,
    new_text: &str,
    logs: &mut Vec<Log>,
) -> u8 {
    if args.print {
        print!("{}", &new_text);
    } else if args.check && text != new_text {
        record_file_log(logs, Error, file, "Incorrect formatting.");
        return 1;
    } else if text != new_text {
        write_file(file, new_text);
    }
    0
}
