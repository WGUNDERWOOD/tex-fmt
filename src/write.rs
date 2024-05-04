use crate::fs;
use std::env::temp_dir;
use std::path;

pub fn backup_file(filename: &str) {
    let filepath = path::Path::new(&filename).canonicalize().unwrap();
    let mut filebak = temp_dir();
    filebak.push("tex-fmt");
    fs::create_dir_all(&filebak).unwrap();
    filebak.push(filepath.file_name().unwrap());
    fs::copy(filepath.clone(), &filebak).unwrap();
}

pub fn write_file(filename: &str, new_file: &str) {
    let filepath = path::Path::new(&filename).canonicalize().unwrap();
    fs::write(filepath, new_file).unwrap();
}
