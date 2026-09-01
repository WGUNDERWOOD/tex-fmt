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
    source_file: &PathBuf,
    target_file: &PathBuf,
    config_file: Option<&PathBuf>,
    cli_file: Option<&PathBuf>,
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
    args.config = config_file.cloned();
    let config = get_config(&args).unwrap();
    let config_args = get_config_args(config).unwrap();
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
            source_file.to_str().unwrap().yellow().bold(),
            target_file.to_str().unwrap().yellow().bold()
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
            }
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
    source_file: &PathBuf,
    target_file: &PathBuf,
    config_file: Option<&PathBuf>,
    cli_file: Option<&PathBuf>,
) -> bool {
    let mut pass = true;
    if !test_file(target_file, target_file, config_file, cli_file) {
        print!(
            "{}",
            format!(
                "Config file: {config_file:?}\n\
            CLI file: {cli_file:?}\n\
            "
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
                "Config file: {config_file:?}\n\
            CLI file: {cli_file:?}\n\
            "
            )
            .yellow()
            .bold()
        );
        pass = false;
    }
    pass
}

fn run_tests_in_dir(test_dir: &fs::DirEntry) -> bool {
    let mut pass = true;
    let config_file = get_config_file(test_dir);
    let cli_file = get_cli_file(test_dir);
    let source_dir = test_dir.path().join("source/");
    let source_files = read_files_from_dir(&source_dir);
    let target_dir = test_dir.path().join("target/");
    let target_files = read_files_from_dir(&target_dir);

    // Source and target file names should match
    #[allow(clippy::manual_assert)]
    if source_files != target_files {
        panic!("Source and target file names differ for {test_dir:?}")
    }

    // Test file formatting
    for file in source_files {
        let source_file = test_dir.path().join("source").join(&file);
        let target_file = test_dir.path().join("target").join(&file);

        // If both config and cli exist, either alone should work
        if config_file.is_some() && cli_file.is_some() {
            pass &= test_source_target(
                &source_file,
                &target_file,
                config_file.as_ref(),
                None,
            );
            pass &= test_source_target(
                &source_file,
                &target_file,
                None,
                cli_file.as_ref(),
            );
        }

        // Pass both config and cli, even if one or more are None
        pass &= test_source_target(
            &source_file,
            &target_file,
            config_file.as_ref(),
            cli_file.as_ref(),
        );
    }

    pass
}

#[test]
fn test_all() {
    let mut pass = true;
    let test_dirs = fs::read_dir("./tests/").unwrap();
    for test_dir in test_dirs {
        pass &= run_tests_in_dir(&test_dir.unwrap());
    }

    assert!(pass);
}

#[test]
#[ignore = "no specific subset of tests to run"]
fn test_subset() {
    let test_names = [
        "wrap_chars",
        //"cv",
        //"short_document",
        //"wrap",
        //"verb",
        //"tables",
    ];
    let mut pass = true;
    let test_dirs = fs::read_dir("./tests/").unwrap().filter(|d| {
        test_names.iter().any(|t| {
            d.as_ref()
                .unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .contains(t)
        })
    });
    for test_dir in test_dirs {
        pass &= run_tests_in_dir(&test_dir.unwrap());
    }
    assert!(pass);
}

/// Run `get_config_args` on raw config file contents, as if read from disk
fn get_config_args_from_str(
    contents: &str,
) -> Result<Option<OptionArgs>, String> {
    let config = Some((
        PathBuf::from("tex-fmt.toml"),
        "tex-fmt.toml".to_string(),
        contents.to_string(),
    ));
    get_config_args(config)
}

#[test]
fn test_config_wraplen_wrong_type() {
    let result = get_config_args_from_str("wraplen = \"80\"\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("wraplen"));
}

#[test]
fn test_config_wrap_chars_empty_string() {
    let result = get_config_args_from_str("wrap-chars = [\"\"]\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("wrap-chars"));
}

#[test]
fn test_config_wrap_chars_too_long() {
    let result = get_config_args_from_str("wrap-chars = [\"ab\"]\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("wrap-chars"));
}

#[test]
fn test_config_check_wrong_type() {
    let result = get_config_args_from_str("check = \"yes\"\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("check"));
}

#[test]
fn test_config_invalid_toml_syntax() {
    let result = get_config_args_from_str("this is not valid toml [[[");
    assert!(result.is_err());
}

#[test]
fn test_config_valid() {
    let result = get_config_args_from_str(
        "wraplen = 80\ncheck = true\nwrap-chars = [\" \", \",\"]\n",
    );
    let args = result.unwrap().unwrap();
    assert_eq!(args.wraplen, Some(80));
    assert_eq!(args.check, Some(true));
    assert_eq!(args.wrap_chars, vec![' ', ',']);
}

#[test]
fn test_config_tabsize_out_of_range() {
    // tabsize is a u8; 300 doesn't fit.
    let result = get_config_args_from_str("tabsize = 300\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("tabsize"));
}

#[test]
fn test_config_wraplen_float() {
    let result = get_config_args_from_str("wraplen = 80.5\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("wraplen"));
}

#[test]
fn test_config_wraplen_negative() {
    // wraplen is a usize; negative doesn't fit.
    let result = get_config_args_from_str("wraplen = -1\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("wraplen"));
}

#[test]
fn test_config_unreadable_file() {
    // get_config (not get_config_args): exercises the file-read error path
    // directly, since get_config_args_from_str always hands back contents
    // that were already "read" successfully.
    let mut option_args = OptionArgs::new();
    option_args.config =
        Some(PathBuf::from("/nonexistent/dir/tex-fmt.toml"));
    let result = get_config(&option_args);
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("failed to read"));
}

#[test]
fn test_config_lists_wrong_type() {
    let result = get_config_args_from_str("lists = \"itemize\"\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("lists"));
}

#[test]
fn test_config_lists_non_string_entry() {
    let result = get_config_args_from_str("lists = [\"itemize\", 5]\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("lists"));
}

#[test]
fn test_config_verbosity_unrecognized() {
    let result = get_config_args_from_str("verbosity = \"debug\"\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("verbosity"));
}

#[test]
fn test_config_tabchar_unrecognized() {
    let result = get_config_args_from_str("tabchar = \"spaces\"\n");
    let message = result.expect_err("expected an error, not a panic");
    assert!(message.contains("tabchar"));
}
