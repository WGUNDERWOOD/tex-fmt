use clap::{value_parser, Arg, ArgAction, Command};
use std::path::PathBuf;
use ArgAction::{Append, SetTrue};

/// Construct the CLI command
#[allow(clippy::too_many_lines)]
#[must_use]
pub fn get_cli_command() -> Command {
    Command::new("tex-fmt")
        .author("William George Underwood, wg.underwood13@gmail.com")
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .before_help(format!("tex-fmt {}", clap::crate_version!()))
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .action(SetTrue)
                .help("Check formatting, do not modify files"),
        )
        .arg(
            Arg::new("print")
                .short('p')
                .long("print")
                .action(SetTrue)
                .help("Print to stdout, do not modify files"),
        )
        .arg(
            Arg::new("fail-on-change")
                .short('f')
                .long("fail-on-change")
                .action(SetTrue)
                .help("Format files and return non-zero exit code if files are modified")
        )
        .arg(
            Arg::new("nowrap")
                .short('n')
                .long("nowrap")
                .action(SetTrue)
                .help("Do not wrap long lines"),
        )
        .arg(
            Arg::new("wraplen")
                .short('l')
                .long("wraplen")
                .value_name("N")
                .value_parser(value_parser!(u8))
                .help("Line length for wrapping [default: 80]"),
        )
        .arg(
            Arg::new("tabsize")
                .short('t')
                .long("tabsize")
                .value_name("N")
                .value_parser(value_parser!(u8))
                .help("Number of characters to use as tab size [default: 2]"),
        )
        .arg(
            Arg::new("usetabs")
                .long("usetabs")
                .action(SetTrue)
                .help("Use tabs instead of spaces for indentation"),
        )
        .arg(
            Arg::new("stdin")
                .short('s')
                .long("stdin")
                .action(SetTrue)
                .help("Process stdin as a single file, output to stdout"),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("PATH")
                .value_parser(value_parser!(PathBuf))
                .help("Path to config file")
        )
        .arg(
            Arg::new("noconfig")
                .long("noconfig")
                .action(SetTrue)
                .help("Do not read any config file"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(SetTrue)
                .help("Show info messages"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(SetTrue)
                .help("Hide warning messages"),
        )
        .arg(
            Arg::new("trace")
                .long("trace")
                .action(SetTrue)
                .help("Show trace messages"),
        )
        .arg(
            Arg::new("completion")
                .long("completion")
                .value_name("SHELL")
                .value_parser(value_parser!(Shell))
                .help("Generate shell completion script")
        )
        .arg(
            Arg::new("man")
                .long("man")
                .action(SetTrue)
                .help("Generate man page"),
        )
        .arg(
            Arg::new("args")
                .long("args")
                .action(SetTrue)
                .help("Print arguments passed to tex-fmt and exit"),
        )
        .arg(
            Arg::new("files")
                .action(Append)
                .help("List of files to be formatted"),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .action(SetTrue)
                .help("Recursively search for files to format")
        )
}
