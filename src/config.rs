use std::path::PathBuf;
use std::env;

enum ConfigFile {
    Named(PathBuf),
    Dir(PathBuf),
    Git(PathBuf),
    //Home(PathBuf),
    //Default,
}

fn get_named_config_file(config: &str) -> ConfigFile {
    ConfigFile::Named(config.into())
}

fn get_dir_config_file() -> ConfigFile {
    let dir = env::current_dir().unwrap();
    ConfigFile::Dir(dir)
}
