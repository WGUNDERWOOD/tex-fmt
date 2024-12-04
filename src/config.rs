//! Read arguments from a config file

use crate::args::*;
use dirs::config_dir;
use log::LevelFilter;
use std::env::current_dir;
use std::fs::{metadata, read_to_string};
use std::path::PathBuf;
use toml::Table;

/// Config file name
const CONFIG: &str = "tex-fmt.toml";

/// Try finding a config file in various sources
fn resolve_config_path(args: &OptionArgs) -> Option<PathBuf> {
    // Named path passed as cli arg
    if args.config.is_some() {
        return args.config.clone();
    };
    // Config file in current directory
    if let Ok(mut config) = current_dir() {
        config.push(CONFIG);
        if config.exists() {
            return Some(config);
        };
    }
    // Config file at git repository root
    if let Some(mut config) = find_git_root() {
        config.push(CONFIG);
        if config.exists() {
            return Some(config);
        };
    }
    // Config file in user home config directory
    if let Some(mut config) = config_dir() {
        config.push("tex-fmt");
        config.push(CONFIG);
        if config.exists() {
            return Some(config);
        };
    }
    None
}

/// Get the git repository root directory
fn find_git_root() -> Option<PathBuf> {
    let mut depth = 0;
    let mut current_dir = current_dir().unwrap();
    while depth < 100 {
        depth += 1;
        if metadata(current_dir.join(".git"))
            .map(|m| m.is_dir())
            .unwrap_or(false)
        {
            return Some(current_dir);
        }
        if !current_dir.pop() {
            break;
        }
    }
    None
}

/// Parse arguments from a config file path
pub fn get_config_args(args: &OptionArgs) -> Option<OptionArgs> {
    let config_path = resolve_config_path(args);
    #[allow(clippy::question_mark)]
    if config_path.is_none() {
        return None;
    };
    let config_string = config_path
        .clone()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let config = read_to_string(config_path.clone().unwrap()).unwrap();
    let config = config.parse::<Table>().unwrap_or_else(|_| {
        panic!("Failed to read config file at {config_string}")
    });

    let verbosity = match config.get("verbosity").map(|x| x.as_str().unwrap()) {
        Some("error") => Some(LevelFilter::Error),
        Some("quiet") => Some(LevelFilter::Error),
        Some("warn") => Some(LevelFilter::Warn),
        Some("verbose") => Some(LevelFilter::Info),
        Some("trace") => Some(LevelFilter::Trace),
        _ => None,
    };

    let tabchar = match config.get("tabchar").map(|x| x.as_str().unwrap()) {
        Some("tab") => Some(TabChar::Tab),
        Some("space") => Some(TabChar::Space),
        _ => None,
    };

    let args = OptionArgs {
        check: config.get("check").map(|x| x.as_bool().unwrap()),
        print: config.get("print").map(|x| x.as_bool().unwrap()),
        wrap: config.get("wrap").map(|x| x.as_bool().unwrap()),
        verbosity,
        files: vec![],

        stdin: config.get("stdin").map(|x| x.as_bool().unwrap()),
        tabsize: config
            .get("tabsize")
            .map(|x| x.as_integer().unwrap().try_into().unwrap()),

        tabchar,
        wraplen: config
            .get("wraplen")
            .map(|x| x.as_integer().unwrap().try_into().unwrap()),
        wrapmin: config
            .get("wrapmin")
            .map(|x| x.as_integer().unwrap().try_into().unwrap()),
        config: config_path,
    };
    Some(args)
}
