use clap::Parser;
use log::Level::Error;
use std::fs;

mod colors;
mod comments;
mod format;
mod ignore;
mod indent;
mod leave;
mod logging;
mod parse;
mod regexes;
mod subs;
mod wrap;
mod write;
use crate::format::*;
use crate::logging::*;
use crate::parse::*;
use crate::write::*;

const TAB: i8 = 2;

#[cfg(test)]
mod tests;

fn main() {
    let mut args = Cli::parse();
    args.resolve();
    init_logger(&args);

    for file in &args.files {
        let mut logs = Vec::<Log>::new();
        let extension_valid = check_extension_valid(file);
        if extension_valid {
            let text = fs::read_to_string(file).unwrap();
            let new_text = format_file(&text, file, &args, &mut logs);
            if args.print {
                println!("{}", &new_text);
            } else {
                write_file(file, &new_text);
            }
        } else {
            record_log(
                &mut logs,
                Error,
                None,
                file.to_string(),
                None,
                None,
                "File type invalid.".to_string(),
            );
        };

        print_logs(&args, logs);
    }
}
