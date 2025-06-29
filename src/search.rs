use crate::regexes::EXTENSIONS;
use ignore::Walk;
use std::path::PathBuf;

/// Find files recursively and append
///
/// # Panics
///
/// This function panics when a file name cannot be converted to a string.
pub fn find_files(dir: &PathBuf, files: &mut Vec<PathBuf>) {
    // Recursive walk of passed directory (ignore errors, symlinks and non-dirs)
    if dir.is_dir() {
        for entry in Walk::new(dir).filter_map(std::result::Result::ok) {
            // If entry is file and has accepted extension, push to files
            if entry.file_type().unwrap().is_file() {
                let file = entry.path();
                if EXTENSIONS.iter().any(|e| file.ends_with(e)) {
                    files.push(file.to_path_buf());
                }
            }
        }
    }
}
