use crate::args::*;
use std::path::PathBuf;
use std::env::current_dir;
use std::fs::read_to_string;
use toml::Table;
use log::LevelFilter;

fn resolve_config_path(args: &OptionArgs) -> Option<PathBuf> {
    // Named path passed as cli arg
    if args.config.is_some() {
        return args.config.clone()
    };
    // Config file in current directory
    let mut config = current_dir().unwrap();
    config.set_file_name("tex-fmt.toml");
    if config.exists() {
        return Some(config)
    };
    // TODO Read from git repo
    // TODO Read from user home config directory
    None
}

pub fn get_config_args(args: &OptionArgs) -> Option<OptionArgs> {
    let config = resolve_config_path(args);
    if config.is_none() {
        return None
    };
    let config = read_to_string(config.unwrap()).unwrap();
    let config = config.parse::<Table>().unwrap();

    let verbosity = match config.get("verbosity").map(|x| x.as_str().unwrap()) {
        Some("trace") => Some(LevelFilter::Trace),
        Some("verbose") => Some(LevelFilter::Info),
        Some("quiet") => Some(LevelFilter::Error),
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
        tabsize: config.get("tabsize").map(|x| x.as_integer().unwrap()
                                      .try_into().unwrap()),

        tabchar,
        wraplen: config.get("wraplen").map(|x| x.as_integer().unwrap()
                                      .try_into().unwrap()),
        wrapmin: config.get("wrapmin").map(|x| x.as_integer().unwrap()
                                      .try_into().unwrap()),
        config: None,
    };
    Some(args)
}
