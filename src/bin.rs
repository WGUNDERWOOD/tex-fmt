//! tex-fmt
//! An extremely fast LaTeX formatter written in Rust

#![warn(missing_docs)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]

use log::LevelFilter;
use std::process::ExitCode;
use tex_fmt::args::get_args;
use tex_fmt::format::run;
use tex_fmt::logging::{init_logger, print_logs, Log};

fn main() -> ExitCode {
    let mut args = match get_args() {
        Ok(args) => args,
        Err(message) => {
            // Verbosity isn't resolved yet, so fall back to the default.
            // There's no single file this error is about (it may span
            // several candidate config paths), so print it directly rather
            // than through record_file_log/print_logs, which always renders
            // a (possibly empty) file name before the message.
            init_logger(LevelFilter::Warn);
            log::error!("tex-fmt: {message}");
            return ExitCode::from(1);
        }
    };
    init_logger(args.verbosity);

    let mut logs = Vec::<Log>::new();
    let mut exit_code = args.resolve(&mut logs);

    if exit_code == 0 {
        exit_code = run(&args, &mut logs);
    }

    print_logs(&mut logs);
    ExitCode::from(exit_code)
}
