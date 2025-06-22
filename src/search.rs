use crate::regexes::EXTENSIONS;
use ignore::Walk;
use std::path::PathBuf;

pub fn find_files(dir: PathBuf, files: &mut Vec<String>) {
    // Recursive walk of passed directory (ignore errors, symlinks and non-dirs)
    if dir.is_dir() {
        for entry in Walk::new(dir).into_iter().filter_map(|e| e.ok()) {
            // If entry is file and has accepted extension, push to files
            if entry.file_type().unwrap().is_file() {
                let file = entry.path().to_str().unwrap();
                if EXTENSIONS.iter().any(|e| file.ends_with(e)) {
                    files.push(file.to_string());
                }
            }
        }
    }
}
