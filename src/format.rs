use crate::colors::*;
use crate::indent::*;
use crate::subs::*;
use crate::wrap::*;
use crate::Cli;

const MAX_WRAP_TRY: u8 = 10;

fn apply_wraps(file: &str, args: &Cli) -> String {
    let mut wrap_tries = 0;
    let mut new_file = file.to_string();
    let mut old_file: String = "".to_string();
    while needs_wrap(&new_file)
        && wrap_tries < MAX_WRAP_TRY
        && new_file != old_file
    {
        log::info!("Wrapping pass number {}", wrap_tries + 1);
        old_file = new_file.clone();
        wrap_tries += 1;
        new_file = wrap(&new_file);
        new_file = remove_trailing_spaces(&new_file);
        new_file = apply_indent(&new_file, args);
    }
    for (i, line) in new_file.lines().enumerate() {
        if line_needs_wrap(line) {
            log::warn!("Line {} cannot be wrapped: {}{:.50}...",
                       i, WHITE, line);
        }
    }
    new_file
}

pub fn format_file(file: &str, args: &Cli) -> String {
    let mut new_file = remove_extra_newlines(file);
    new_file = begin_end_environments_new_line(&new_file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    new_file = apply_indent(&new_file, args);
    new_file = apply_wraps(&new_file, args);

    new_file
}
