use clap::Parser;
use core::cmp::max;
use lazy_static::lazy_static;
use regex::Regex;
use std::env::temp_dir;
use std::fs;
use std::path;

const TAB: i32 = 2;
const OPENS: [char; 3] = ['(', '[', '{'];
const CLOSES: [char; 3] = [')', ']', '}'];
const LISTS: [&str; 4] = ["itemize", "enumerate", "description", "inlineroman"];

const YELLOW: &str = "\x1b[33m\x1b[1m";
const PINK: &str = "\x1b[35m\x1b[1m";
const RESET: &str = "\x1b[00m\x1b[0m";

#[derive(Parser)]
struct Cli {
    #[arg(long, short, help = "Print to stdout, do not modify files")]
    print: bool,
    #[arg(
        long,
        short,
        help = "Debug mode, disable checks and do not modify files"
    )]
    debug: bool,
    #[arg(required = true)]
    filenames: Vec<String>,
}

lazy_static! {
    static ref RE_NEWLINES: Regex = Regex::new(r"\n\n\n+").unwrap();
    static ref RE_TABS: Regex = Regex::new(r"\t").unwrap();
    static ref RE_TRAIL: Regex = Regex::new(r" +\n").unwrap();
    static ref RE_PERCENT: Regex = Regex::new(r"\\\%").unwrap();
    static ref RE_COMMENT: Regex = Regex::new(r"\%.*").unwrap();
    static ref RE_ITEM: Regex = Regex::new(r".*\\item.*").unwrap();
    static ref RE_DOCUMENT_BEGIN: Regex =
        Regex::new(r".*\\begin\{document\}.*").unwrap();
    static ref RE_DOCUMENT_END: Regex =
        Regex::new(r".*\\end\{document\}.*").unwrap();
    static ref RE_ENV_BEGIN: Regex =
        Regex::new(r".*\\begin\{[a-z\*]*\}.*").unwrap();
    static ref RE_ENV_END: Regex =
        Regex::new(r".*\\end\{[a-z\*]*\}.*").unwrap();
    static ref RE_LISTS_BEGIN: Vec<Regex> = LISTS
        .iter()
        .map(|l| Regex::new(&format!(r".*\\begin\{{{}}}.*", l)).unwrap())
        .collect();
    static ref RE_LISTS_END: Vec<Regex> = LISTS
        .iter()
        .map(|l| Regex::new(&format!(r".*\\end\{{{}}}.*", l)).unwrap())
        .collect();
}

fn remove_extra_newlines(file: &str) -> String {
    RE_NEWLINES.replace_all(file, "\n\n").to_string()
}

fn remove_tabs(file: &str) -> String {
    let replace = (0..TAB).map(|_| " ").collect::<String>();
    RE_TABS.replace_all(file, replace).to_string()
}

fn remove_trailing_spaces(file: &str) -> String {
    RE_TRAIL.replace_all(file, "\n").to_string()
}

fn remove_comment(line: &str) -> String {
    let new_line = RE_PERCENT.replace_all(line, "").to_string();
    RE_COMMENT.replace_all(&new_line, "").to_string()
}

fn get_back(line: &str) -> i32 {
    // no deindentation for ending document
    if RE_DOCUMENT_END.is_match(line) {
        return 0;
    };

    // list environments get double indents for indenting items
    for re_list_end in RE_LISTS_END.iter() {
        if re_list_end.is_match(line) {
            return 2;
        };
    }

    // other environments get single indents
    if RE_ENV_END.is_match(line) {
        return 1;
    };

    // deindent items to make the rest of item environment appear indented
    if RE_ITEM.is_match(line) {
        return 1;
    };

    let mut back: i32 = 0;
    let mut cumul: i32 = 0;
    for c in line.chars() {
        cumul -= OPENS.contains(&c) as i32;
        cumul += CLOSES.contains(&c) as i32;
        back = max(cumul, back);
    }
    back
}

fn get_diff(line: &str) -> i32 {
    // no indentation for document
    if RE_DOCUMENT_BEGIN.is_match(line) {
        return 0;
    };
    if RE_DOCUMENT_END.is_match(line) {
        return 0;
    };

    // list environments get double indents
    let mut diff: i32 = 0;
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

    // delimiters
    for c in OPENS {
        diff += line.chars().filter(|&x| x == c).count() as i32;
    }
    for c in CLOSES {
        diff -= line.chars().filter(|&x| x == c).count() as i32;
    }
    diff
}

fn format_file(file: String, debug: bool) -> String {
    // preformat
    let mut new_file = remove_extra_newlines(&file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    let lines: Vec<&str> = new_file.lines().collect();

    // set up variables
    let mut count: i32 = 0;
    let n_lines = lines.len();
    let mut indents: Vec<i32> = vec![0; lines.len()];
    let mut new_lines = vec!["".to_owned(); n_lines];

    // main loop through file
    for i in 0..n_lines {
        // calculate indent
        let line = lines[i];
        let line_strip = &remove_comment(line);
        let back = get_back(line_strip);
        let diff = get_diff(line_strip);
        let indent: i32 = count - back;
        if !debug {
            assert!(indent >= 0)
        };
        indents[i] = indent;
        count += diff;

        // apply indent
        let mut new_line = line.trim_start().to_string();
        if !new_line.is_empty() {
            let n_spaces = indents[i] * TAB;
            let spaces: String = (0..n_spaces).map(|_| " ").collect();
            new_line.insert_str(0, &spaces);
        }
        new_lines[i] = new_line
    }

    // check indents return to zero
    if !debug {
        assert!(indents.first().unwrap() == &0);
        assert!(indents.last().unwrap() == &0);
    }

    // prepare indented file
    let mut new_file = new_lines.join("\n");
    new_file.push('\n');
    new_file
}

#[cfg(test)]
mod tests {
    use crate::fs;
    use crate::format_file;
    #[test]
    fn test() {
        let in_file = fs::read_to_string("test/test_in.tex").expect("");
        let out_file = fs::read_to_string("test/test_out.tex").expect("");
        let fmt_file = format_file(in_file, false);
        assert_eq!(fmt_file, out_file);
    }
}

fn main() {
    // get arguments
    let args = Cli::parse();
    let debug = args.debug;
    let mut print = args.print;
    let filenames = args.filenames;
    if debug {
        print = true;
    };

    // check files are in correct format
    assert!(filenames.iter().all(|f| f.ends_with(".tex")
        || f.ends_with(".bib")
        || f.ends_with(".cls")));

    // print script name
    println!("{}", String::new() + PINK + "tex-fmt" + RESET);

    for filename in filenames {
        // print file name
        if debug {
            println!("{}", String::new() + YELLOW + &filename + RESET);
        }

        // read lines from file
        let file =
            fs::read_to_string(&filename).expect("Should have read the file");

        let new_file = format_file(file, debug);

        if print {
            // print new file
            println!("{}", &new_file);
        } else {
            // backup original file
            let filepath = path::Path::new(&filename).canonicalize().unwrap();
            let mut fileback = temp_dir();
            fileback.push("tex-fmt");
            fs::create_dir_all(&fileback).unwrap();
            fileback.push(filepath.file_name().unwrap());
            fs::copy(filepath.clone(), &fileback).unwrap();

            // write new file
            fs::write(filepath, new_file).unwrap();
        }
    }
}
