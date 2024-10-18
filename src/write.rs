//! Utilities for writing formatted files

use crate::cli::*;
use crate::fs;
use crate::logging::*;
use log::Level::Error;
use std::path;

/// Write a formatted file to disk
fn write_file(file: &str, text: &str) {
    let filepath = path::Path::new(&file).canonicalize().unwrap();
    fs::write(filepath, text).expect("Could not write the file");
}

/// Handle the newly formatted file
pub fn process_output(
    args: &Cli,
    file: &str,
    text: &str,
    new_text: &str,
    exit_code: u8,
    logs: &mut Vec<Log>,
) -> u8 {
    let mut new_exit_code = exit_code;
    if args.print {
        println!("{}", &new_text);
    } else if args.check && text != new_text {
        record_file_log(logs, Error, file, "Incorrect formatting.");
        new_exit_code = 1;
    } else if text != new_text {
        write_file(file, new_text);
    }
    new_exit_code
}
