//! Main library

pub mod args;
pub mod cli;
pub mod comments;
pub mod config;
pub mod format;
pub mod ignore;
pub mod indent;
pub mod logging;
pub mod read;
pub mod regexes;
pub mod subs;
pub mod verbatim;
pub mod wasm;
pub mod wrap;
pub mod write;

#[cfg(test)]
pub mod tests;

#[cfg(any(target_family = "unix", target_family = "wasm"))]
/// Line ending for unix
const LINE_END: &str = "\n";

#[cfg(target_family = "windows")]
/// Line ending for Windows
const LINE_END: &str = "\r\n";
