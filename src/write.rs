use crate::fs;
use std::path;

pub fn write_file(filename: &str, new_file: &str) {
    let filepath = path::Path::new(&filename).canonicalize().unwrap();
    fs::write(filepath, new_file).unwrap();
}
