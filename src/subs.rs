use crate::comments::*;
use crate::format::*;
use crate::ignore::*;
use crate::leave::*;
use crate::logging::*;
use crate::regexes::*;
use crate::Cli;
use crate::{LINE_END, TAB};
use log::Level::Info;

pub fn remove_extra_newlines(text: &str) -> String {
    let double_line_end = format!("{}{}", LINE_END, LINE_END);
    RE_NEWLINES.replace_all(text, double_line_end).to_string()
}

pub fn remove_tabs(text: &str) -> String {
    let replace = (0..TAB).map(|_| " ").collect::<String>();
    text.replace('\t', &replace)
}

pub fn remove_trailing_spaces(text: &str) -> String {
    RE_TRAIL.replace_all(text, LINE_END).to_string()
}

pub fn environments_new_line(
    text: &str,
    file: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> String {
    if args.verbose {
        record_file_log(
            logs,
            Info,
            file,
            "Ensuring environments on new lines.",
        );
    }

    let mut state = State::new();
    let mut new_text = String::with_capacity(text.len());

    for line in text.lines() {
        state.ignore = get_ignore(line, &state, logs, file, false);
        state.leave = get_leave(line, &state, logs, file, true);

        if !state.leave.visual
            && !state.ignore.visual
            && (line.contains(ENV_BEGIN)
                || line.contains(ENV_END)
                || line.contains(ITEM))
        {
            let comment_index = find_comment_index(line);
            let comment = &get_comment(line, comment_index);
            let text = &remove_comment(line, comment_index);
            let text = &RE_ENV_BEGIN_SHARED_LINE
                .replace_all(text, format!("$prev{}$env", LINE_END))
                .to_string();
            let text = &RE_ENV_END_SHARED_LINE
                .replace_all(text, format!("$prev{}$env", LINE_END))
                .to_string();
            let text = &RE_ITEM_SHARED_LINE
                .replace_all(text, format!("$prev{}$env", LINE_END))
                .to_string();
            new_text.push_str(text);
            new_text.push_str(comment);
        } else {
            new_text.push_str(line);
        }
        new_text.push_str(LINE_END);
    }
    new_text
}
