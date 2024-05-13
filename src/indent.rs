use crate::colors::*;
use crate::comments::*;
use crate::parse::*;
use crate::regexes::*;
use crate::TAB;
use core::cmp::max;

const OPENS: [char; 3] = ['(', '[', '{'];
const CLOSES: [char; 3] = [')', ']', '}'];

#[derive(Debug)]
struct Indent {
    actual: i8,
    visual: i8,
}

impl Indent {
    fn new() -> Self {
        Indent {
            actual: 0,
            visual: 0,
        }
    }
}

/// calculate total indentation change due to current line
fn get_diff(line: &str) -> i8 {
    // documents get no global indentation
    if RE_DOCUMENT_BEGIN.is_match(line) || RE_DOCUMENT_END.is_match(line) {
        return 0;
    };

    // list environments get double indents
    let mut diff: i8 = 0;
    for re_list_begin in RE_LISTS_BEGIN.iter() {
        if re_list_begin.is_match(line) {
            diff += 1
        };
    }
    for re_list_end in RE_LISTS_END.iter() {
        if re_list_end.is_match(line) {
            diff -= 1
        };
    }

    // other environments get single indents
    if RE_ENV_BEGIN.is_match(line) {
        diff += 1
    };
    if RE_ENV_END.is_match(line) {
        diff -= 1
    };

    // indent for delimiters
    diff += line.chars().filter(|x| OPENS.contains(x)).count() as i8;
    diff -= line.chars().filter(|x| CLOSES.contains(x)).count() as i8;

    diff
}

/// calculate dedentation for current line compared to previous
fn get_back(line: &str) -> i8 {
    // documents get no global indentation
    if RE_DOCUMENT_END.is_match(line) {
        return 0;
    };

    // list environments get double indents for indenting items
    for re_list_end in RE_LISTS_END.iter() {
        if re_list_end.is_match(line) {
            return 2;
        };
    }

    let mut back: i8 = 0;
    let mut cumul: i8 = 0;

    // delimiters
    for c in line.chars() {
        cumul -= OPENS.contains(&c) as i8;
        cumul += CLOSES.contains(&c) as i8;
        back = max(cumul, back);
    }

    // other environments get single indents
    if RE_ENV_END.is_match(line) {
        back += 1;
    };

    // deindent items to make the rest of item environment appear indented
    if RE_ITEM.is_match(line) {
        back += 1;
    };

    back
}

fn get_indent(line: &str, prev_indent: Indent) -> Indent {
    let diff = get_diff(line);
    let back = get_back(line);
    let actual = prev_indent.actual + diff;
    let visual = prev_indent.actual - back;
    Indent { actual, visual }
}

pub fn apply_indent(file: &str, args: &Cli, log: bool) -> String {
    log::info!("Indenting file");
    let mut indent = Indent::new();
    let mut new_file = "".to_owned();
    let mut verbatim_count = 0;

    for (i, line) in file.lines().enumerate() {
        if RE_VERBATIM_BEGIN.is_match(line) {
            verbatim_count += 1;
        }
        if verbatim_count == 0 {
            // calculate indent
            let comment_index = find_comment_index(line);
            let line_strip = remove_comment(line, comment_index);
            indent = get_indent(line_strip, indent);
            log::info!(
                "Indent line {}: actual = {}, visual = {}:{} {}",
                i,
                indent.actual,
                indent.visual,
                WHITE,
                line,
            );
            if !args.debug {
                if (indent.actual < 0 || indent.visual < 0) && log {
                    log::error!(
                        "Line {} requests a negative indent: {}{}",
                        i,
                        WHITE,
                        line
                    );
                    indent.actual = 0;
                }
            };

            // apply indent
            let mut new_line = line.trim_start().to_string();
            if !new_line.is_empty() {
                let n_spaces = indent.visual * TAB;
                let spaces: String = (0..n_spaces).map(|_| " ").collect();
                new_line.insert_str(0, &spaces);
            }
            new_file.push_str(&new_line);
        } else {
            new_file.push_str(line);
        }
        new_file.push('\n');
        if RE_VERBATIM_BEGIN.is_match(line) {
            verbatim_count += 1;
        }
    }

    // check indents return to zero
    if !args.debug && log {
        if indent.actual != 0 || indent.visual != 0 {
            log::error!("Indent does not return to zero at end of file");
        }
    }

    new_file
}
