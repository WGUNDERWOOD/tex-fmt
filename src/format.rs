use crate::indent::*;
use crate::subs::*;
use crate::wrap::*;
use crate::Cli;

const MAX_WRAP_TRY: u8 = 5;

fn apply_wraps(file: &str, args: &Cli) -> String {
    let mut wrap_tries = 0;
    let mut new_file = file.to_string();
    let mut warn_string: Option<String> = None;
    while needs_wrap(&new_file) && wrap_tries < MAX_WRAP_TRY {
        log::info!("Pass number {}", wrap_tries);
        wrap_tries += 1;
        (new_file, warn_string) = wrap(&new_file);
        new_file = remove_trailing_spaces(&new_file);
        new_file = apply_indent(&new_file, args);
    }
    if let Some(s) = warn_string {
        log::warn!("{}", s)
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
