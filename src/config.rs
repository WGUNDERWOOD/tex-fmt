use crate::Cli;
use std::path::PathBuf;
use std::env;

pub enum ConfigFile {
    Named(PathBuf),
    Dir(PathBuf),
    //Git(PathBuf),
    //Home(PathBuf),
    Default,
}

pub fn get_config(args: &Cli) -> ConfigFile {
    // Config file named in cli args
    if let Some(config) = &args.config {
        return ConfigFile::Named(config.into())
    }
    // Config file in current directory
    let mut dir_config = env::current_dir().unwrap();
    dir_config.set_file_name("tex-fmt.toml");
    if dir_config.exists() {
        return ConfigFile::Dir(dir_config)
    }
    // TODO read from git repo
    // TODO read from user home config directory
    ConfigFile::Default
}

//fn get_named_config_file(config: &str) -> ConfigFile {
    //ConfigFile::Named(config.into())
//}

//fn get_dir_config_file() -> ConfigFile {
    //let dir = env::current_dir().unwrap();
    //ConfigFile::Dir(dir)
//}
