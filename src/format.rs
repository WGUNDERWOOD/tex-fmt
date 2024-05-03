use crate::indent::*;
use crate::subs::*;
use crate::wrap::*;

const MAX_WRAP_TRY: u8 = 3;

pub fn format_file(file: &str, debug: bool) -> String {
    let mut new_file = remove_extra_newlines(file);
    new_file = begin_end_environments_new_line(&new_file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    new_file = apply_indent(&new_file, debug);

    let mut wrap_tries = 0;
    while needs_wrap(&file) && wrap_tries < MAX_WRAP_TRY {
        wrap_tries += 1;
        new_file = wrap(&new_file);
        new_file = remove_trailing_spaces(&new_file);
        new_file = apply_indent(&new_file, debug);
    };

    new_file
}
