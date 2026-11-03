//! Main library

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/WGUNDERWOOD/tex-fmt/main/extra/logo.svg"
)]
#![warn(clippy::pedantic)]

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
pub mod search;
pub mod subs;
pub mod table;
pub mod verbatim;
pub mod wrap;
pub mod write;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(test)]
pub mod tests;

/// Line ending used while formatting
const LINE_END: &str = "\n";

#[cfg(any(target_family = "unix", target_family = "wasm"))]
/// Line ending for unix, used for input without a line ending
const NATIVE_LINE_END: &str = "\n";

#[cfg(target_family = "windows")]
/// Line ending for Windows, used for input without a line ending
const NATIVE_LINE_END: &str = "\r\n";
