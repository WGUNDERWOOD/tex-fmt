use crate::indent::*;
use crate::subs::*;
use crate::wrap::*;
use crate::Cli;

const MAX_TRIES: u8 = 10;

fn apply_passes(file: &str, filename: &str, args: &Cli) -> String {
    let mut new_file = apply_indent(file, filename, args);
    let mut finished = false;
    let mut tries = 0;

    while needs_wrap(&new_file) && !finished && tries < MAX_TRIES {
        let old_file = new_file.clone();
        new_file = wrap(&new_file, filename);
        new_file = remove_trailing_spaces(&new_file);
        new_file = apply_indent(&new_file, filename, args);
        tries += 1;
        if new_file == old_file {
            finished = true;
        }
    }

    // check indents return to zero
    if new_file.lines().last().unwrap().starts_with(' ') {
        log::error!("Indent does not return to zero at end of file");
    }

    new_file
}

pub fn format_file(file: &str, filename: &str, args: &Cli) -> String {
    let mut new_file = remove_extra_newlines(file);
    new_file = begin_end_environments_new_line(&new_file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    new_file = apply_passes(&new_file, filename, args);
    new_file
}
