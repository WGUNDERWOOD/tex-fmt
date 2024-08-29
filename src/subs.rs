use crate::comments::*;
use crate::format::*;
use crate::ignore::*;
use crate::leave::*;
use crate::logging::*;
use crate::regexes::*;
//use crate::Cli;
use crate::TAB;
//use log::Level::Info;

pub fn remove_extra_newlines(text: &str) -> String {
    RE_NEWLINES.replace_all(text, "\n\n").to_string()
}

pub fn remove_tabs(text: &str) -> String {
    let replace = (0..TAB).map(|_| " ").collect::<String>();
    RE_TABS.replace_all(text, replace).to_string()
}

pub fn remove_trailing_spaces(text: &str) -> String {
    RE_TRAIL.replace_all(text, "\n").to_string()
}

pub fn environments_new_line(
    text: &str,
    file: &str,
    //args: &Cli,
    logs: &mut Vec<Log>,
) -> String {
    //if args.verbose {
    //record_log(
    //logs,
    //Info,
    //None,
    //file.to_string(),
    //None,
    //None,
    //"Ensuring environments on new lines.".to_string(),
    //);
    //}

    //let mut ignore = Ignore::new();
    //let mut leave = Leave::new();
    let mut state = State::new();
    let mut new_text = String::with_capacity(text.len());

    for (linum, line) in text.lines().enumerate() {
        state.ignore = get_ignore(line, &state, logs, false);
        state.leave = get_leave(line, &state);

        if !state.leave.visual
            && !state.ignore.visual
            && (RE_ENV_BEGIN.is_match(line)
                || RE_ENV_END.is_match(line)
                || RE_ITEM.is_match(line))
        {
            let comment_index = find_comment_index(line);
            let comment = &get_comment(line, comment_index);
            let text = &remove_comment(line, comment_index);
            let text = &RE_ENV_BEGIN_SHARED_LINE
                .replace_all(text, "$prev\n$env")
                .to_string();
            let text = &RE_ENV_END_SHARED_LINE
                .replace_all(text, "$prev\n$env")
                .to_string();
            let text = &RE_ITEM_SHARED_LINE
                .replace_all(text, "$prev\n$env")
                .to_string();
            new_text.push_str(text);
            new_text.push_str(comment);
        } else {
            new_text.push_str(line);
        }
        new_text.push('\n');
    }
    new_text
}
