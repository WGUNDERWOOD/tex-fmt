use crate::args::*;
use crate::cli::*;
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
    config_file: &Option<PathBuf>,
    cli_file: &Option<PathBuf>,
) -> bool {
    // Get arguments from CLI file
    let mut args = match cli_file {
        Some(f) => {
            let cli_args = fs::read_to_string(f).unwrap();
            let cli_args = cli_args.strip_suffix("\n").unwrap();
            let mut cli_args: Vec<&str> = cli_args.split_whitespace().collect();
            cli_args.insert(0, "tex-fmt");
            let matches =
                get_cli_command().try_get_matches_from(&cli_args).unwrap();
            get_cli_args(Some(matches))
        }
        None => OptionArgs::new(),
    };

    // Merge arguments from config file
    args.config = config_file.clone();
    let config = get_config(&args);
    let config_args = get_config_args(config);
    if let Some(c) = config_args {
        args.merge(c);
    }

    // Merge in default arguments
    args.merge(OptionArgs::default());
    let args = Args::from(args);

    // Run tex-fmt
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
    let mut files: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .collect();
    files.sort();
    files
}

fn get_config_file(dir: &fs::DirEntry) -> Option<PathBuf> {
    let config_file = dir.path().join("tex-fmt.toml");
    if config_file.exists() {
        Some(config_file)
    } else {
        None
    }
}

fn get_cli_file(dir: &fs::DirEntry) -> Option<PathBuf> {
    let cli_file = dir.path().join("cli.txt");
    if cli_file.exists() {
        Some(cli_file)
    } else {
        None
    }
}

fn test_source_target(
    source_file: &str,
    target_file: &str,
    config_file: &Option<PathBuf>,
    cli_file: &Option<PathBuf>,
) -> bool {
    let mut pass = true;
    if !test_file(target_file, target_file, config_file, cli_file) {
        print!(
            "{}",
            format!(
                "Config file: {:?}\n\
            CLI file: {:?}\n\
            ",
                config_file, cli_file
            )
            .yellow()
            .bold()
        );
        pass = false;
    }

    if !test_file(source_file, target_file, config_file, cli_file) {
        print!(
            "{}",
            format!(
                "Config file: {:?}\n\
            CLI file: {:?}\n\
            ",
                config_file, cli_file
            )
            .yellow()
            .bold()
        );
        pass = false;
    }
    pass
}

#[test]
fn test() {
    let mut pass = true;
    let test_dirs = fs::read_dir("./tests/").unwrap();
    for test_dir in test_dirs {
        let test_dir = test_dir.unwrap();
        let config_file = get_config_file(&test_dir);
        let cli_file = get_cli_file(&test_dir);
        let source_dir = test_dir.path().join("source/");
        let source_files = read_files_from_dir(&source_dir);
        let target_dir = test_dir.path().join("target/");
        let target_files = read_files_from_dir(&target_dir);

        // Source and target file names should match
        if source_files != target_files {
            panic!("Source and target file names differ for {:?}", test_dir)
        }

        // Test file formatting
        for file in source_files {
            let source_file = test_dir.path().join("source").join(file.clone());
            let source_file = source_file.to_str().unwrap();
            let target_file = test_dir.path().join("target").join(file.clone());
            let target_file = target_file.to_str().unwrap();

            // If both config and cli exist, either alone should work
            if config_file.is_some() && cli_file.is_some() {
                pass &= test_source_target(
                    source_file,
                    target_file,
                    &config_file,
                    &None,
                );
                pass &= test_source_target(
                    source_file,
                    target_file,
                    &None,
                    &cli_file,
                );
            }

            // Pass both config and cli, even if one or more are None
            pass &= test_source_target(
                source_file,
                target_file,
                &config_file,
                &cli_file,
            );
        }
    }

    assert!(pass)
}
