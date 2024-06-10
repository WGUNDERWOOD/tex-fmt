use crate::colors::*;
use crate::format_file;
use crate::fs;
use crate::logging::*;
use crate::Cli;
use similar::{ChangeTag, TextDiff};

fn test_file(in_file: &str, out_file: &str) -> bool {
    let args = Cli::new();
    let mut logs = Vec::<Log>::new();
    let in_text = fs::read_to_string(&in_file).unwrap();
    let out_text = fs::read_to_string(&out_file).unwrap();
    let fmt_in_text = format_file(&in_text, &in_file, &args, &mut logs);

    if fmt_in_text != out_text {
        println!(
            "{}fail {}{} {}-> {}{}",
            RED, YELLOW, in_file, RESET, YELLOW, out_file
        );
        let diff = TextDiff::from_lines(&fmt_in_text, &out_text);
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => print!(
                    "{}@ {}: {}- {}{}",
                    PURPLE,
                    change.old_index().unwrap(),
                    RED,
                    change,
                    RESET
                ),
                ChangeTag::Insert => print!(
                    "{}@ {}: {}+ {}{}",
                    PURPLE,
                    change.new_index().unwrap(),
                    GREEN,
                    change,
                    RESET
                ),
                ChangeTag::Equal => {}
            };
        }
    }

    fmt_in_text == out_text
}

fn read_files_from_dir(dir: &str) -> Vec<String> {
    fs::read_dir(dir)
        .unwrap()
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .collect()
}

#[test]
fn test_in() {
    let in_files = read_files_from_dir("./tests/in/");
    let mut fail = false;
    for file in in_files {
        if !test_file(
            &format!("tests/in/{}", file),
            &format!("tests/out/{}", file),
        ) {
            fail = true
        }
    }
    if fail {
        panic!("Some tests failed")
    }
}

#[test]
fn test_out() {
    let out_files = read_files_from_dir("./tests/out/");
    let mut fail = false;
    for file in out_files {
        if !test_file(
            &format!("tests/out/{}", file),
            &format!("tests/out/{}", file),
        ) {
            fail = true
        }
    }
    if fail {
        panic!("Some tests failed")
    }
}
