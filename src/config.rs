use crate::Cli;
use path_absolutize::*;
//use config::Config;
use std::fs;
use std::path::PathBuf;
use std::env;
use std::collections::HashMap;
use toml::Table;

#[derive(Debug)]
pub enum ConfigPath {
    Named(PathBuf),
    Dir(PathBuf),
    //Git(PathBuf),
    //Home(PathBuf),
    Default,
}

pub fn get_config_path(args: &Cli) -> ConfigPath {
    // Config file named in cli args
    if let Some(config) = &args.config {
        let config_path: PathBuf = config.into();
        let config_path = config_path.absolutize().expect("Read the config path").into();
        return ConfigPath::Named(config_path);
    }
    // Config file in current directory
    let mut dir_config = env::current_dir().unwrap();
    dir_config.set_file_name("tex-fmt.toml");
    if dir_config.exists() {
        return ConfigPath::Dir(dir_config)
    }
    // TODO read from git repo
    // TODO read from user home config directory
    ConfigPath::Default
}

pub fn read_config_file(args: &Cli) -> Cli {
    let config_path = get_config_path(args);
    let default_config: Cli = Cli::new();
    let file_config: Table = match config_path {
        ConfigPath::Named(p) => {
            dbg!(&p);
            let contents = fs::read_to_string(p).unwrap();
            //contents.parse::<Table>().unwrap()
            dbg!(&contents);
            toml::from_str(&contents).unwrap()
        },
        //Config::builder()
        //.add_source(config::File::with_name(p.to_str().unwrap()))
        //.build()
        //.unwrap(),
        _ => todo!()
    };
    dbg!(&file_config);
    //for key in file_config.keys() {
        //dbg!(key);
    //}
    //dbg!(file_config);
    //config
    args.clone()
}

//#[serde(default, with = "date_serde")]

//fn get_named_config_file(config: &str) -> ConfigFile {
    //ConfigFile::Named(config.into())
//}

//fn get_dir_config_file() -> ConfigFile {
    //let dir = env::current_dir().unwrap();
    //ConfigFile::Dir(dir)
//}
