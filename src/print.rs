const PINK: &str = "\x1b[35m\x1b[1m";
const RESET: &str = "\x1b[00m\x1b[0m";
const YELLOW: &str = "\x1b[33m\x1b[1m";

pub fn print_script_name() {
    println!("{}", String::new() + PINK + "tex-fmt" + RESET);
}

pub fn print_file_name(filename: &str) {
    println!("{}", String::new() + YELLOW + filename + RESET);
}

pub fn print_file(new_file: &str) {
    println!("{}", new_file);
}
