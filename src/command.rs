use clap::{value_parser, Command, Arg, ArgAction};
use ArgAction::{Append, SetTrue};
use std::path::PathBuf;

/// Construct the CLI command
#[allow(clippy::too_many_lines)]
fn get_cli_command() -> Command {
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
                .value_parser(value_parser!(u8))
                .help("Line length for wrapping [default: 80]"),
        )
        .arg(
            Arg::new("tabsize")
                .short('t')
                .long("tabsize")
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
                .value_parser(value_parser!(PathBuf))
                .help("Path to configuration file")
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
                .value_parser(value_parser!(Shell))
                .value_name("shell")
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
}
