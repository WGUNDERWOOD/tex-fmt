use crate::cli::*;
use crate::config::*;
use merge::Merge;
use log::LevelFilter;
use std::path::PathBuf;

#[derive(Clone, Debug, Merge)]
pub struct OptionArgs {
    pub check: Option<bool>,
    pub print: Option<bool>,
    pub wrap: Option<bool>,
    pub verbosity: Option<LevelFilter>,
    #[merge(strategy = merge::vec::append)]
    pub files: Vec<String>,
    pub stdin: Option<bool>,
    pub tabsize: Option<u8>,
    pub tabchar: Option<TabChar>,
    pub wraplen: Option<u8>,
    pub wrapmin: Option<u8>,
    pub config: Option<PathBuf>,
}

pub struct Args {
    pub check: bool,
    pub print: bool,
    pub wrap: bool,
    pub verbosity: LevelFilter,
    pub files: Vec<String>,
    pub stdin: bool,
    pub tabsize: u8,
    pub tabchar: TabChar,
    pub wraplen: u8,
    pub wrapmin: u8,
    pub config: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TabChar {
    Tab,
    Space,
}

impl Default for OptionArgs {
    fn default() -> OptionArgs {
        OptionArgs {
            check: Some(false),
            print: Some(false),
            wrap: Some(true),
            verbosity: Some(LevelFilter::Warn),
            files: vec![],
            stdin: Some(false),
            tabsize: Some(2),
            tabchar: Some(TabChar::Space),
            wraplen: Some(80),
            wrapmin: Some(70),
            config: None,
        }
    }
}

pub fn get_args() -> Args {
    let mut args = get_cli_args();
    let config_args = get_config_args(&args);
    if let Some(c) = config_args {
        args.merge(c);
    }
    args.merge(OptionArgs::default());
    Args::from(args)
}

//impl OptionArgs {
    // TODO
    // fn resolve()
//}

impl Args {
    fn from(args: OptionArgs) -> Self {
        Args {
            check: args.check.unwrap(),
            print: args.print.unwrap(),
            wrap: args.wrap.unwrap(),
            verbosity: args.verbosity.unwrap(),
            files: args.files,
            stdin: args.stdin.unwrap(),
            tabsize: args.tabsize.unwrap(),
            tabchar: args.tabchar.unwrap(),
            wraplen: args.wraplen.unwrap(),
            wrapmin: args.wrapmin.unwrap(),
            config: args.config,
        }
    }
}
