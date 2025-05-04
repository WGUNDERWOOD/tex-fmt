//! Read arguments from a config file

use crate::args::{OptionArgs, TabChar};
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
    // Do not read config file
    if args.noconfig == Some(true) {
        return None;
    }
    // Named path passed as cli arg
    if args.config.is_some() {
        return args.config.clone();
    }
    // Config file in current directory
    if let Ok(mut config) = current_dir() {
        config.push(CONFIG);
        if config.exists() {
            return Some(config);
        }
    }
    // Config file at git repository root
    if let Some(mut config) = find_git_root() {
        config.push(CONFIG);
        if config.exists() {
            return Some(config);
        }
    }
    // Config file in user home config directory
    if let Some(mut config) = config_dir() {
        config.push("tex-fmt");
        config.push(CONFIG);
        if config.exists() {
            return Some(config);
        }
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

/// Read content from a config file path
///
/// # Panics
///
/// This function panics if the config file cannot be read.
#[must_use]
pub fn get_config(args: &OptionArgs) -> Option<(PathBuf, String, String)> {
    let config_path = resolve_config_path(args);
    config_path.as_ref()?;
    let config_path_string = config_path
        .clone()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let config = read_to_string(config_path.clone().unwrap()).unwrap();
    Some((config_path.unwrap(), config_path_string, config))
}

fn parse_array_string(name: &str, config: &Table) -> Vec<String> {
    config
        .get(name)
        .and_then(|v| v.as_array())
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect()
}

fn string_to_char(s: &str) -> char {
    let mut chars = s.chars();
    let c = chars.next().expect("String is empty");
    if chars.next().is_some() {
        panic!("String contains more than one character");
    }
    c
}

/// Parse arguments from a config file path
///
/// # Panics
///
/// This function panics if the config file cannot be parsed into TOML
#[must_use]
pub fn get_config_args(
    config: Option<(PathBuf, String, String)>,
) -> Option<OptionArgs> {
    config.as_ref()?;
    let (config_path, config_path_string, config) = config.unwrap();
    let config = config.parse::<Table>().unwrap_or_else(|_| {
        panic!("Failed to read config file at {config_path_string}")
    });

    let verbosity = match config.get("verbosity").map(|x| x.as_str().unwrap()) {
        Some("error" | "quiet") => Some(LevelFilter::Error),
        Some("warn") => Some(LevelFilter::Warn),
        Some("info" | "verbose") => Some(LevelFilter::Info),
        Some("trace") => Some(LevelFilter::Trace),
        _ => None,
    };

    let tabchar = match config.get("tabchar").map(|x| x.as_str().unwrap()) {
        Some("tab") => Some(TabChar::Tab),
        Some("space") => Some(TabChar::Space),
        _ => None,
    };

    // Read wrap_chars to Vec<char> not Vec<String>
    let wrap_chars: Vec<char> = parse_array_string("wrap-chars", &config)
        .iter()
        .map(|c| string_to_char(c))
        .collect();

    let args = OptionArgs {
        check: config.get("check").map(|x| x.as_bool().unwrap()),
        print: config.get("print").map(|x| x.as_bool().unwrap()),
        fail_on_change: config
            .get("fail-on-change")
            .map(|x| x.as_bool().unwrap()),
        wrap: config.get("wrap").map(|x| x.as_bool().unwrap()),
        wraplen: config
            .get("wraplen")
            .map(|x| x.as_integer().unwrap().try_into().unwrap()),
        wrapmin: config
            .get("wrapmin")
            .map(|x| x.as_integer().unwrap().try_into().unwrap()),
        tabsize: config
            .get("tabsize")
            .map(|x| x.as_integer().unwrap().try_into().unwrap()),
        tabchar,
        stdin: config.get("stdin").map(|x| x.as_bool().unwrap()),
        config: Some(config_path),
        noconfig: None,
        lists: parse_array_string("lists", &config),
        verbatims: parse_array_string("verbatims", &config),
        no_indent_envs: parse_array_string("no-indent-envs", &config),
        wrap_chars,
        verbosity,
        arguments: None,
        files: vec![],
    };
    Some(args)
}
