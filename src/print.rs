use crate::colors::*;

pub fn print_script_name() {
    println!("{}{}{}", PINK, "tex-fmt", RESET);
}

pub fn print_filename(filename: &str) {
    println!("{}{}{}{}{}", PINK, "tex-fmt ", PURPLE, filename, RESET);
}

pub fn print_file(new_file: &str) {
    println!("{}", new_file);
}
