use crate::args::*;
use crate::config::*;
use crate::format::format_file;
use crate::logging::*;
use colored::Colorize;
use merge::Merge;
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::path::PathBuf;

fn test_file(
    source_file: &str,
    target_file: &str,
    config_file: Option<PathBuf>,
) -> bool {
    // Get arguments from config file if it exists
    let mut args = OptionArgs::new();
    args.config = config_file;
    let config = get_config(&args);
    let config_args = get_config_args(config);
    if let Some(c) = config_args {
        args.merge(c);
    }
    args.merge(OptionArgs::default());
    let args = Args::from(args);

    let mut logs = Vec::<Log>::new();
    let source_text = fs::read_to_string(source_file).unwrap();
    let target_text = fs::read_to_string(target_file).unwrap();
    let fmt_source_text =
        format_file(&source_text, source_file, &args, &mut logs);

    if fmt_source_text != target_text {
        println!(
            "{} {} -> {}",
            "fail".red().bold(),
            source_file.yellow().bold(),
            target_file.yellow().bold()
        );
        let diff = TextDiff::from_lines(&fmt_source_text, &target_text);
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => print!(
                    "{} {}",
                    format!("@ {:>3}:", change.old_index().unwrap())
                        .blue()
                        .bold(),
                    format!("- {change}").red().bold(),
                ),
                ChangeTag::Insert => print!(
                    "{} {}",
                    format!("@ {:>3}:", change.new_index().unwrap())
                        .blue()
                        .bold(),
                    format!("+ {change}").green().bold(),
                ),
                ChangeTag::Equal => {}
            };
        }
    }

    fmt_source_text == target_text
}

fn read_files_from_dir(dir: &PathBuf) -> Vec<String> {
    fs::read_dir(dir)
        .unwrap()
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .collect()
}

#[test]
fn test_source() {
    let test_dirs = fs::read_dir("./tests/").unwrap();
    for test_dir in test_dirs {
        let test_dir = test_dir.unwrap();
        let source_dir = test_dir.path().join("source/");
        let source_files = read_files_from_dir(&source_dir);
        for file in source_files {
            let in_file = test_dir.path().join("source").join(file.clone());
            let out_file = test_dir.path().join("target").join(file.clone());
            let config_file = test_dir.path().join("tex-fmt.toml");
            let config_file = if config_file.exists() {
                Some(config_file)
            } else {
                None
            };
            if !test_file(
                in_file.to_str().unwrap(),
                out_file.to_str().unwrap(),
                config_file,
            ) {
                panic!("Failed in {file}");
            }
        }
    }
}

#[test]
fn test_target() {
    let test_dirs = fs::read_dir("./tests/").unwrap();
    for test_dir in test_dirs {
        let test_dir = test_dir.unwrap();
        let target_dir = test_dir.path().join("target/");
        let target_files = read_files_from_dir(&target_dir);
        for file in target_files {
            let in_file = test_dir.path().join("target").join(file.clone());
            let config_file = test_dir.path().join("tex-fmt.toml");
            let config_file = if config_file.exists() {
                Some(config_file)
            } else {
                None
            };
            if !test_file(
                in_file.to_str().unwrap(),
                in_file.to_str().unwrap(),
                config_file,
            ) {
                panic!("Failed in {file}");
            }
        }
    }
}

#[test]
#[ignore]
fn test_short() {
    let source_files = vec!["wrap/source/wrap.tex"];
    let target_files = vec!["wrap/target/wrap.tex"];
    let mut fail = false;
    for i in 0..source_files.len() {
        let source_file = source_files[i];
        let target_file = target_files[i];
        if !test_file(
            &format!("tests/{source_file}"),
            &format!("tests/{target_file}"),
            None,
        ) {
            fail = true;
        }
        if !test_file(
            &format!("tests/{target_file}"),
            &format!("tests/{target_file}"),
            None,
        ) {
            fail = true;
        }
    }
    assert!(!fail, "Some tests failed");
}
