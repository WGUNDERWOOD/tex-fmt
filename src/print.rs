use crate::colors::*;

pub fn print_script_name() {
    println!("{}", String::new() + PINK + "tex-fmt" + RESET);
}

pub fn print_file_name(filename: &str) {
    println!("{}", String::new() + YELLOW + filename + RESET);
}

pub fn print_file(new_file: &str) {
    println!("{}", new_file);
}
