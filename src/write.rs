//! Utilities for writing formatted files

use crate::args::Args;
use crate::logging::{record_file_log, Log};
use log::Level::{Error, Info};
use std::fs;
use std::path::PathBuf;

/// Write a formatted file to disk
fn write_file(file: &PathBuf, text: &str) {
    let filepath = file.canonicalize().unwrap();
    fs::write(filepath, text).expect("Could not write the file");
}

/// Handle the newly formatted file
pub fn process_output(
    args: &Args,
    file: &PathBuf,
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
        if args.fail_on_change {
            record_file_log(logs, Info, file, "Fixed incorrect formatting.");
            return 1;
        }
    }
    0
}
