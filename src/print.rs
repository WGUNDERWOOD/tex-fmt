use crate::colors::*;

pub fn print_script_name() {
    println!("{}tex-fmt{}", PINK, RESET);
}

pub fn print_filename(filename: &str) {
    println!("{}tex-fmt {}{}{}", PINK, PURPLE, filename, RESET);
}

pub fn print_file(new_file: &str) {
    println!("{}", new_file);
}
