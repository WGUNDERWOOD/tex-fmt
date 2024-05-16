use crate::colors::*;

pub fn print_script_name() {
    println!("{}", String::new() + PINK + "tex-fmt" + RESET);
}

pub fn print_filename(filename: &str) {
    println!("{}", String::new() + PURPLE + filename + RESET);
}

pub fn print_file(new_file: &str) {
    println!("{}", new_file);
}
