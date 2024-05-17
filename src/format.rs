//use crate::colors::*;
use crate::indent::*;
use crate::logging::*;
use crate::subs::*;
use crate::wrap::*;
use crate::Cli;
//use log::Level::Warn;

const MAX_TRIES: u8 = 10;

fn apply_passes(
    file: &str,
    args: &Cli,
    filename: &str,
    logs: &mut Vec<Log>,
) -> String {
    let mut new_logs: Vec<Log> = vec![];
    let mut new_file = apply_indent(file, args, filename, &mut new_logs);
    let mut finished = false;
    let mut tries = 0;

    while needs_wrap(&new_file) && !finished && tries < MAX_TRIES {
        new_logs.clear();
        let old_file = new_file.clone();
        new_file = wrap(&new_file, filename, &mut new_logs);
        new_file = remove_trailing_spaces(&new_file);
        new_file = apply_indent(&new_file, args, filename, &mut new_logs);
        tries += 1;
        if new_file == old_file {
            finished = true;
        }
    }

    // check indents return to zero
    //if new_file.lines().last().unwrap().starts_with(' ') {
    //log::error!("Indent does not return to zero at end of file");
    //}

    // TODO move this logging into wrap.rs
    //if needs_wrap(&new_file) {
    //for (i, line) in new_file.lines().enumerate() {
    //if line_needs_wrap(line) {
    //record_log(
    //Warn,
    //i,
    //format!(
    //" Line {}: cannot be wrapped: {}{:.50}",
    //i, WHITE, line
    //),
    //);
    //}
    //}
    //}

    logs.append(&mut new_logs);
    new_file
}

pub fn format_file(
    file: &str,
    args: &Cli,
    filename: &str,
    logs: &mut Vec<Log>,
) -> String {
    let mut new_file = remove_extra_newlines(file);
    new_file = begin_end_environments_new_line(&new_file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    new_file = apply_passes(&new_file, args, filename, logs);
    new_file
}
