use crate::comments::*;
use crate::ignore::*;
use crate::leave::*;
use crate::logging::*;
use crate::regexes::*;
use crate::Cli;
use crate::TAB;
use log::Level::Info;

pub fn remove_extra_newlines(file: &str) -> String {
    RE_NEWLINES.replace_all(file, "\n\n").to_string()
}

pub fn remove_tabs(file: &str) -> String {
    let replace = (0..TAB).map(|_| " ").collect::<String>();
    RE_TABS.replace_all(file, replace).to_string()
}

pub fn remove_trailing_spaces(file: &str) -> String {
    RE_TRAIL.replace_all(file, "\n").to_string()
}

pub fn environments_new_line(
    file: &str,
    filename: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> String {
    if args.verbose {
        record_log(
            logs,
            Info,
            None,
            filename.to_string(),
            None,
            None,
            "Ensuring environments on new lines.".to_string(),
        );
    }

    let mut ignore = Ignore::new();
    let mut leave = Leave::new();
    let mut new_file = String::with_capacity(file.len());

    for (linum, line) in file.lines().enumerate() {
        ignore = get_ignore(line, linum, ignore, filename, logs, None, false);
        leave = get_leave(line, linum, leave, filename, logs, None, false);

        if !leave.visual
            && !ignore.visual
            && (RE_ENV_BEGIN.is_match(line)
                || RE_ENV_END.is_match(line)
                || RE_ITEM.is_match(line))
        {
            let comment_index = find_comment_index(line);
            let comment = get_comment(line, comment_index);
            let text = remove_comment(line, comment_index);
            let text = &RE_ENV_BEGIN_SHARED_LINE
                .replace_all(text, "$prev\n$env")
                .to_string();
            let text = &RE_ENV_END_SHARED_LINE
                .replace_all(text, "$prev\n$env")
                .to_string();
            let text = &RE_ITEM_SHARED_LINE
                .replace_all(text, "$prev\n$env")
                .to_string();
            new_file.push_str(text);
            new_file.push_str(comment);
        } else {
            new_file.push_str(line);
        }
        new_file.push('\n');
    }
    new_file
}
