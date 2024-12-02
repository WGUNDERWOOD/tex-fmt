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
    // TODO implement
    let matches = get_cli_args();
    let mut cli_args = from_arg_matches(&matches);
    let config: Table;
    if let Some(c) = &cli_args.config {
        config = read_config(c.to_path_buf());
        let config_args = from_table(&config);
        cli_args.merge(config_args.clone());
    }
    cli_args.merge(Args::default());
    cli_args
}
