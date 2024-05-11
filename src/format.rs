use crate::indent::*;
use crate::subs::*;
use crate::wrap::*;
use crate::Cli;

const MAX_WRAP_TRY: u8 = 3;

pub fn format_file(file: &str, args: &Cli) -> String {
    let mut new_file = remove_extra_newlines(file);
    new_file = begin_end_environments_new_line(&new_file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    new_file = apply_indent(&new_file, args);

    let mut wrap_tries = 0;
    while needs_wrap(&new_file) && wrap_tries < MAX_WRAP_TRY {
        wrap_tries += 1;
        new_file = wrap(&new_file);
        new_file = remove_trailing_spaces(&new_file);
        new_file = apply_indent(&new_file, args);
    }

    new_file
}
