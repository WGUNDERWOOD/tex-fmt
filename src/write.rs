//! Utilities for writing formatted files

use crate::fs;
use std::path;

/// Write a formatted file to disk
pub fn write_file(file: &str, text: &str) {
    let filepath = path::Path::new(&file).canonicalize().unwrap();
    fs::write(filepath, text).expect("Could not write the file");
}
