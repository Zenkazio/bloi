use crate::prelude::*;

use std::{
    env::{self},
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub store_dir: PathBuf,
}

impl Config {
    pub fn default_config() -> Result<Self> {
        Ok(Self {
            store_dir: get_home_path()?.join(".dotfiles"),
        })
    }
}

pub fn load_config() -> Result<Config> {
    if let Ok(full_path) = full_config_file_path() {
        if let Ok(json) = fs::read_to_string(full_path) {
            if let Ok(config) = serde_json::from_str(&json) {
                return Ok(config);
            }
        }
    }
    Ok(create_config(&full_config_file_path()?)?)
}

pub fn create_config(path: &Path) -> Result<Config> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).expect("could not create dirs");
    }
    save_config(&Config::default_config()?).expect("could not save file");
    Config::default_config()
}

pub fn save_config(config: &Config) -> Result<()> {
    if let Ok(json) = serde_json::to_string_pretty(config) {
        if let Ok(mut file) = fs::File::create(full_config_file_path()?) {
            file.write_all(json.as_bytes())
                .expect("file could not be written :(");
            return Ok(());
        }
    }
    Err(Error::Generic(
        "Something happend while parsing the config".to_string(),
    ))
}

pub fn get_home_path() -> Result<PathBuf> {
    if let Ok(home) = env::var("HOME") {
        return Ok(PathBuf::from(home));
    }
    //this would be better XDG_CONFIG_DIR ect.
    Err(Error::Generic("$HOME was not found".to_string()))
}

pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_home_path()?.join(".config"))
}

pub fn full_config_file_path() -> Result<PathBuf> {
    Ok(get_config_path()?.join("bloi").join("config.json"))
}
