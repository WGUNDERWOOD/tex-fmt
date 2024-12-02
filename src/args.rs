#[derive(Clone, Debug, Merge)]
pub struct Args {
    pub check: Option(bool),
    pub print: Option(bool),
    pub wrap: Option(bool),
    pub verbosity: Option(Log::Level),
    pub files: Vec<String>,
    pub stdin: Option(bool),
    pub tabsize: Option(u8),
    pub tabchar: Option(TabChar),
    pub wraplen: Option(u8),
    pub wrapmin: Option(u8),
    pub config: Option(String),
}

impl Default for Args {
    fn default() -> Args {
        Args {
            check: Some(false),
            print: Some(false),
            wrap: Some(true),
            verbosity: Some(Log::Level::Warn),
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

fn get_args() -> Args {
    let mut args = get_cli_args();
    let config_args = get_config_args(args);
    args.merge(config_args);
    args.merge(Args::default());
    args
}

impl Args {
    // TODO
    // fn resolve()
}
