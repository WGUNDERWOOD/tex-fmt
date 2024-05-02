use crate::indent::*;
use crate::subs::*;
use crate::comments::*;
use crate::TAB;

pub fn format_file(file: &str, debug: bool) -> String {
    // preformat
    let mut new_file = remove_extra_newlines(file);
    new_file = begin_end_environments_new_line(&new_file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    let lines: Vec<&str> = new_file.lines().collect();

    // set up variables
    let n_lines = lines.len();
    let mut indent = Indent {
        actual: 0,
        visual: 0,
    };
    let mut new_lines = vec!["".to_owned(); n_lines];

    // main loop through file
    for i in 0..n_lines {
        // calculate indent
        let line = lines[i];
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
        new_lines[i] = new_line
    }

    // check indents return to zero
    if !debug {
        assert!(indent.actual == 0);
        assert!(indent.visual == 0);
    }

    // prepare indented file
    let mut new_file = new_lines.join("\n");
    new_file.push('\n');
    new_file
}
