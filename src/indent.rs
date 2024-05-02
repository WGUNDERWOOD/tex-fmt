use crate::comments::*;
use crate::regexes::*;
use crate::TAB;
use core::cmp::max;

const OPENS: [char; 3] = ['(', '[', '{'];
const CLOSES: [char; 3] = [')', ']', '}'];

#[derive(Debug)]
pub struct Indent {
    /// actual running indentation count at end of current line
    pub actual: i8,
    /// visual indentation of current line
    pub visual: i8,
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
pub fn get_diff(line: &str) -> i8 {
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
    for c in OPENS {
        diff += line.chars().filter(|&x| x == c).count() as i8;
    }
    for c in CLOSES {
        diff -= line.chars().filter(|&x| x == c).count() as i8;
    }

    diff
}

/// calculate dedentation for current line compared to previous
pub fn get_back(line: &str) -> i8 {
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

pub fn get_indent(line: &str, prev_indent: Indent) -> Indent {
    let diff = get_diff(line);
    let back = get_back(line);
    let actual = prev_indent.actual + diff;
    let visual = prev_indent.actual - back;
    Indent { actual, visual }
}

pub fn apply_indent(file: &str, debug: bool) -> String {
    let lines: Vec<&str> = file.lines().collect();
    let mut indent = Indent::new();
    let mut new_file = "".to_owned();

    for (i, line) in lines.iter().enumerate() {
        // calculate indent
        let comm = find_comment(line);
        let line_strip = remove_comment(line, &comm);
        indent = get_indent(line_strip, indent);
        if !debug {
            assert!(indent.actual >= 0, "line {}: {}", i, line);
            assert!(indent.visual >= 0, "line {}: {}", i, line);
        };

        // apply indent
        let mut new_line = line.trim_start().to_string();
        if !new_line.is_empty() {
            let n_spaces = indent.visual * TAB;
            let spaces: String = (0..n_spaces).map(|_| " ").collect();
            new_line.insert_str(0, &spaces);
        }
        new_file.push_str(&new_line);
        new_file.push('\n');
    }

    // check indents return to zero
    if !debug {
        assert!(indent.actual == 0);
        assert!(indent.visual == 0);
    }

    new_file
}
