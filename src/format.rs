use crate::indent::*;
//use crate::program::*;
//use crate::file::*;
//use crate::logging::*;
//use crate::subs::*;
//use crate::wrap::*;
use crate::ignore::*;
use crate::leave::*;
//use crate::parse::*;
//use log::Level::{Info};

//const MAX_PASS: usize = 10;

pub fn format_file(text: &str, file: &str) -> String {
    let mut state = State::new();
    let mut old_lines: Vec<&str> = text.lines().rev().collect();
    let mut queue: Vec<String> = vec![];
    let mut new_text: String = "".to_string();
    dbg!(file);

    //dbg!(old_lines);

    loop {
        if !queue.is_empty() {
            // process the queue
            let mut line = queue.pop().unwrap();
            (line, state) = apply_indent(&line, &state);
            dbg!(&line);
            println!("\n\n");
            new_text.push_str(&line);
            new_text.push('\n');
        } else if !old_lines.is_empty() {
            // move the next line into the queue
            let line: String = old_lines.pop().unwrap().to_string();
            //dbg!(&line);
            queue.push(line);
        } else {
            break
        }
        //dbg!(&queue);
    }

    new_text
}

#[derive(Clone)]
pub struct State {
    pub linum_old: usize,
    pub linum_new: usize,
    pub ignore: Ignore,
    pub indent: Indent,
    pub leave: Leave,
}

impl State {
    pub fn new() -> Self {
        State {
            linum_old: 0,
            linum_new: 0,
            ignore: Ignore::new(),
            indent: Indent::new(),
            leave: Leave::new(),
        }
    }
}

// TODO write this function
//fn apply_indent_wrap(
    //text: &str,
    //file: &str,
    //args: &Cli,
    //logs: &mut Vec<Log>,
//) -> String {

    //let old_lines: Vec<&str> = text.lines().collect();
    //let queue: Vec<String> = vec![];
    //let new_lines: Vec<String> = vec![];

    //text.to_string()
//}

/*
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
*/

/*
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
    new_text = apply_indent_wrap(&new_text, file, args, logs);
    new_text
}
*/
