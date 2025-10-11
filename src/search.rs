use crate::regexes::EXTENSIONS;
use ignore::Walk;
use std::path::PathBuf;

/// Find files recursively and append
///
/// # Panics
///
/// This function panics when a file extension cannot be converted to a string.
pub fn find_files(dir: &PathBuf, files: &mut Vec<PathBuf>) {
    // Recursive walk of passed directory (ignore errors, symlinks and non-dirs)
    for entry in Walk::new(dir).filter_map(std::result::Result::ok) {
        // If entry is file and has accepted extension, push to files
        if entry.file_type().unwrap().is_file() {
            let file = entry.path();
            if let Some(ext_osstr) = file.extension() {
                let ext = ext_osstr.to_str().unwrap();
                if EXTENSIONS.contains(&ext) {
                    files.push(file.to_path_buf());
                }
            }
        }
    }
}
