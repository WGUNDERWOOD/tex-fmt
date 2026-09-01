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
    let mut current_dir = current_dir().ok()?;
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
/// # Errors
///
/// Returns an error message if the config file path is not valid UTF-8,
/// or if the config file exists but cannot be read.
pub fn get_config(
    args: &OptionArgs,
) -> Result<Option<(PathBuf, String, String)>, String> {
    let Some(config_path) = resolve_config_path(args) else {
        return Ok(None);
    };
    let config_path_string = config_path
        .clone()
        .into_os_string()
        .into_string()
        .map_err(|_| "config: file path is not valid UTF-8".to_string())?;
    let config = read_to_string(&config_path).map_err(|e| {
        format!("config: failed to read file at {config_path_string}: {e}")
    })?;
    Ok(Some((config_path, config_path_string, config)))
}

/// Get an array-of-strings field from a config table, erroring if the field
/// exists but isn't an array, or if any of its entries isn't a string
fn parse_array_string(config: &Table, name: &str) -> Result<Vec<String>, String> {
    let Some(v) = config.get(name) else {
        return Ok(vec![]);
    };
    let arr = v.as_array().ok_or_else(|| {
        format!("{name}: expected an array, found a {}", v.type_str())
    })?;
    arr.iter()
        .map(|e| {
            e.as_str().map(String::from).ok_or_else(|| {
                format!(
                    "{name}: expected an array of strings, found a {} entry",
                    e.type_str()
                )
            })
        })
        .collect()
}

/// Get an optional boolean field from a config table
fn expect_bool(config: &Table, name: &str) -> Result<Option<bool>, String> {
    config.get(name).map_or(Ok(None), |v| {
        v.as_bool().map(Some).ok_or_else(|| {
            format!("{name}: expected a boolean, found a {}", v.type_str())
        })
    })
}

/// Get an optional string field from a config table
fn expect_str<'a>(
    config: &'a Table,
    name: &str,
) -> Result<Option<&'a str>, String> {
    config.get(name).map_or(Ok(None), |v| {
        v.as_str().map(Some).ok_or_else(|| {
            format!("{name}: expected a string, found a {}", v.type_str())
        })
    })
}

/// Get an optional integer field from a config table, converting it to `T`
fn expect_integer<T: TryFrom<i64>>(
    config: &Table,
    name: &str,
) -> Result<Option<T>, String> {
    config.get(name).map_or(Ok(None), |v| {
        let i = v.as_integer().ok_or_else(|| {
            format!("{name}: expected an integer, found a {}", v.type_str())
        })?;
        T::try_from(i)
            .map(Some)
            .map_err(|_| format!("{name}: integer {i} is out of range"))
    })
}

fn string_to_char(name: &str, s: &str) -> Result<char, String> {
    let mut chars = s.chars();
    let c = chars
        .next()
        .ok_or_else(|| format!("{name}: entry is an empty string"))?;
    if chars.next().is_some() {
        return Err(format!("{name}: entry {s:?} has more than one character"));
    }
    Ok(c)
}

/// Parse arguments from a config file path
///
/// # Errors
///
/// Returns an error message if the config file cannot be parsed as TOML,
/// or if any of its fields have the wrong type or an invalid value.
pub fn get_config_args(
    config: Option<(PathBuf, String, String)>,
) -> Result<Option<OptionArgs>, String> {
    let Some((config_path, config_path_string, config)) = config else {
        return Ok(None);
    };
    let config = config.parse::<Table>().map_err(|e| {
        format!("config: failed to parse file at {config_path_string}: {e}")
    })?;

    let verbosity = match expect_str(&config, "verbosity")? {
        Some("error" | "quiet") => Some(LevelFilter::Error),
        Some("warn") => Some(LevelFilter::Warn),
        Some("info" | "verbose") => Some(LevelFilter::Info),
        Some("trace") => Some(LevelFilter::Trace),
        Some(other) => {
            return Err(format!(
                "verbosity: unrecognized value {other:?}, expected one of \
                 \"error\"/\"quiet\", \"warn\", \"info\"/\"verbose\", \"trace\""
            ))
        }
        None => None,
    };

    let tabchar = match expect_str(&config, "tabchar")? {
        Some("tab") => Some(TabChar::Tab),
        Some("space") => Some(TabChar::Space),
        Some(other) => {
            return Err(format!(
                "tabchar: unrecognized value {other:?}, expected \"tab\" or \"space\""
            ))
        }
        None => None,
    };

    // Read wrap_chars to Vec<char> not Vec<String>
    let wrap_chars: Vec<char> = parse_array_string(&config, "wrap-chars")?
        .iter()
        .map(|c| string_to_char("wrap-chars", c))
        .collect::<Result<Vec<char>, String>>()?;

    let args = OptionArgs {
        check: expect_bool(&config, "check")?,
        print: expect_bool(&config, "print")?,
        fail_on_change: expect_bool(&config, "fail-on-change")?,
        wrap: expect_bool(&config, "wrap")?,
        wraplen: expect_integer(&config, "wraplen")?,
        wrapmin: expect_integer(&config, "wrapmin")?,
        tabsize: expect_integer(&config, "tabsize")?,
        tabchar,
        stdin: expect_bool(&config, "stdin")?,
        config: Some(config_path),
        noconfig: None,
        lists: parse_array_string(&config, "lists")?,
        verbatims: parse_array_string(&config, "verbatims")?,
        no_indent_envs: parse_array_string(&config, "no-indent-envs")?,
        wrap_chars,
        verbosity,
        arguments: None,
        files: vec![],
        recursive: None,
        format_tables: expect_bool(&config, "format-tables")?,
    };
    Ok(Some(args))
}
