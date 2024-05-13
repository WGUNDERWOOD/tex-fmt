use crate::colors::*;
use crate::indent::*;
use crate::subs::*;
use crate::wrap::*;
use crate::Cli;

const MAX_TRIES: u8 = 10;

fn apply_passes(file: &str, args: &Cli) -> String {
    let (mut new_file, mut indent_errs) = apply_indent(file, args);

    for _ in 0..MAX_TRIES {
        if needs_wrap(&new_file) {
            let old_file = new_file.clone();
            new_file = wrap(&new_file);
            new_file = remove_trailing_spaces(&new_file);
            if new_file == old_file {
                continue;
            }
            (new_file, indent_errs) = apply_indent(&new_file, args);
        } else {
            continue;
        }
    }

    for indent_err in indent_errs {
        let i = indent_err.0;
        let line = new_file.lines().nth(i).unwrap();
        log::error!("Indent is negative on line {}: {}{}", i, WHITE, line);
    }

    // check indents return to zero
    if new_file.lines().last().unwrap().starts_with(' ') {
        log::error!("Indent does not return to zero at end of file");
    }

    if needs_wrap(&new_file) {
        for (i, line) in new_file.lines().enumerate() {
            if line_needs_wrap(line) {
                log::warn!(
                    "Line {} cannot be wrapped: {}{:.50}...",
                    i,
                    WHITE,
                    line
                );
            }
        }
    }
    new_file
}

pub fn format_file(file: &str, args: &Cli) -> String {
    let mut new_file = remove_extra_newlines(file);
    new_file = begin_end_environments_new_line(&new_file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    new_file = apply_passes(&new_file, args);
    new_file
}
