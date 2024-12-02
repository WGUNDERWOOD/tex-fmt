fn resolve_config_path(args: Args) -> Option(PathBuf) {
    // Named path passed as cli arg
    if let Some(config) = args.config {
        return config.absolutize()
    };
    // Config file in current directory
    let mut config = env::current_dir().unwrap();
    config.set_file_name("tex-fmt.toml");
    if config.exists() {
        return config.absolutize();
    };
    // TODO Read from git repo
    // TODO Read from user home config directory
    None
}

fn get_config_args(args: Args) -> Option(Args) {
    let config = resolve_config_path(args);
    if config.is_none() {
        return None
    };
    let config = fs::read_to_string(config).unwrap();
    let config = config.parse::<Table>().unwrap();

    let verbosity = match config.map("verbosity").map(|x| x.as_string().unwrap()) {
        "trace" => Some(LevelFilter::Trace),
        "verbose" => Some(LevelFilter::Info),
        "quiet" => Some(LevelFilter::Error),
        _ => None,
    };

    let tabchar = match config.map("tabchar").map(|x| x.as_string().unwrap()) {
        "tab" => Some(TabChar::Tab),
        "space" => Some(TabChar::Space),
        _ => None,
    };

    let args = Args {
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
