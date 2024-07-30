use crate::indent::*;
use crate::logging::*;
use crate::subs::*;
use crate::wrap::*;
use crate::Cli;
use log::Level::{Info, Warn};

const MAX_PASS: usize = 10;

fn apply_passes(
    text: &str,
    file: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> String {
    let mut new_text = apply_indent(text, file, args, logs, Some(1));
    let mut finished = false;
    let mut pass = 2;

    while !finished && needs_wrap(&new_text) && pass < MAX_PASS + 2 {
        let old_text = new_text.clone();
        new_text = wrap(&new_text, file, logs, Some(pass), args);
        new_text = remove_trailing_spaces(&new_text);
        new_text = apply_indent(&new_text, file, args, logs, Some(pass));
        pass += 1;
        if new_text == old_text {
            finished = true;
        }
    }

    record_log(
        logs,
        Info,
        None,
        file.to_string(),
        None,
        None,
        "Passes completed.".to_string(),
    );

    // check indents return to zero
    if new_text.lines().last().unwrap_or_default().starts_with(' ') {
        record_log(
            logs,
            Warn,
            None,
            file.to_string(),
            None,
            None,
            "Indent does not return to zero.".to_string(),
        );
    }

    new_text
}

pub fn format_file(
    text: &str,
    file: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> String {
    if args.verbose {
        record_log(
            logs,
            Info,
            None,
            file.to_string(),
            None,
            None,
            "Begin formatting.".to_string(),
        );
    }
    let mut new_text = remove_extra_newlines(text);
    new_text = environments_new_line(&new_text, file, args, logs);
    new_text = remove_tabs(&new_text);
    new_text = remove_trailing_spaces(&new_text);
    new_text = apply_passes(&new_text, file, args, logs);
    new_text
}
