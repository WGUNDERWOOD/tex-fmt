use crate::args::*;
use crate::cli::*;
use crate::config::*;
use crate::format::format_file;
use crate::logging::*;
use crate::write::process_output;
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

/// Cleans up a temp file on drop, including on panic/early return, and
/// restores write permission first so removal doesn't itself fail.
#[cfg(unix)]
struct TempFileGuard(PathBuf);

#[cfg(unix)]
impl Drop for TempFileGuard {
    fn drop(&mut self) {
        use std::os::unix::fs::PermissionsExt;
        let writable = std::fs::Permissions::from_mode(0o644);
        let _ = fs::set_permissions(&self.0, writable);
        let _ = fs::remove_file(&self.0);
    }
}

/// Writing to a read-only file should report an error, not panic
#[test]
#[cfg(unix)]
fn test_write_readonly_file_does_not_panic() {
    use std::os::unix::fs::PermissionsExt;

    let path = std::env::temp_dir().join(format!(
        "tex-fmt-test-readonly-{}-{:?}.tex",
        std::process::id(),
        std::thread::current().id()
    ));
    fs::write(&path, "old content").unwrap();
    let _guard = TempFileGuard(path.clone());

    let readonly = std::fs::Permissions::from_mode(0o444);
    fs::set_permissions(&path, readonly).unwrap();

    let mut option_args = OptionArgs::new();
    option_args.merge(OptionArgs::default());
    let args = Args::from(option_args);

    let mut logs = Vec::<Log>::new();
    let exit_code =
        process_output(&args, &path, "old content", "new content", &mut logs);

    // Restore permissions so the contents can be inspected; the guard also
    // restores them on drop, so this is safe to call again there.
    let writable = std::fs::Permissions::from_mode(0o644);
    fs::set_permissions(&path, writable).unwrap();
    let file_contents = fs::read_to_string(&path).unwrap();

    if file_contents == "old content" {
        // The write was actually blocked: assert the failure is reported
        // cleanly instead of panicking.
        assert_eq!(exit_code, 1);
        assert!(logs.iter().any(|l| l.level == log::Level::Error));
    } else {
        // Permission bits weren't enforced (e.g. running as root, or a
        // filesystem/container that ignores them) - the read-only scenario
        // wasn't actually exercised. Still assert the write succeeded
        // cleanly rather than silently asserting nothing.
        eprintln!(
            "test_write_readonly_file_does_not_panic: read-only permission \
             was not enforced (likely running as root); skipping the \
             failure-path assertions, only checking the write succeeded"
        );
        assert_eq!(file_contents, "new content");
        assert_eq!(exit_code, 0);
    }
}
