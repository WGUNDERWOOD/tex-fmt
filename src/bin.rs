//! tex-fmt
//! An extremely fast LaTeX formatter written in Rust

#![warn(missing_docs)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::module_name_repetitions)]

use std::process::ExitCode;
use tex_fmt::args::get_args;
use tex_fmt::format::run;
use tex_fmt::logging::{init_logger, print_logs, Log};

fn main() -> ExitCode {
    let mut args = get_args();
    init_logger(args.verbosity);

    let mut logs = Vec::<Log>::new();
    let mut exit_code = args.resolve(&mut logs);

    if exit_code == 0 {
        exit_code = run(&args, &mut logs);
    }

    print_logs(&mut logs);
    ExitCode::from(exit_code)
}
