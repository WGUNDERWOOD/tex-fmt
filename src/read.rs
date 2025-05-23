//! Utilities for reading files

use crate::logging::{record_file_log, Log};
use crate::regexes::EXTENSIONS;
use log::Level::{Error, Trace};
use std::fs;
use std::io::Read;

/// Add a missing extension and read the file
pub fn read(file: &str, logs: &mut Vec<Log>) -> Option<(String, String)> {
    // Check if file has an accepted extension
    let has_ext = EXTENSIONS.iter().any(|e| file.ends_with(e));
    // If no valid extension, try adding .tex
    let mut new_file = file.to_owned();
    if !has_ext {
        new_file.push_str(".tex");
    }
    if let Ok(text) = fs::read_to_string(&new_file) {
        return Some((new_file, text));
    }
    if has_ext {
        record_file_log(logs, Error, file, "Could not open file.");
    } else {
        record_file_log(logs, Error, file, "File type invalid.");
    }
    None
}

/// Attempt to read from stdin, return filename `<stdin>` and text
pub fn read_stdin(logs: &mut Vec<Log>) -> Option<(String, String)> {
    let mut text = String::new();
    match std::io::stdin().read_to_string(&mut text) {
        Ok(bytes) => {
            record_file_log(
                logs,
                Trace,
                "<stdin>",
                &format!("Read {bytes} bytes."),
            );
            Some((String::from("<stdin>"), text))
        }
        Err(e) => {
            record_file_log(
                logs,
                Error,
                "<stdin>",
                &format!("Could not read from stdin: {e}"),
            );
            None
        }
    }
}
