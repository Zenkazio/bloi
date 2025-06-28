use std::collections::HashSet;
use std::{fs, io::Write, path::PathBuf};

use dirs::{config_dir, home_dir};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub store_path: PathBuf,
    pub adds: HashSet<PathBuf>,
}

impl Config {
    pub fn default_config() -> Result<Self, String> {
        let mut temp = Self {
            store_path: get_store_path()?,
            adds: HashSet::new(),
        };
        temp.adds.insert(get_full_config_path()?);
        Ok(temp)
    }
    pub fn save(&self) -> Result<(), String> {
        if let Ok(json) = serde_json::to_string_pretty(&self) {
            if let Ok(mut file) = fs::File::create(get_full_config_file_path()?) {
                file.write_all(json.as_bytes())
                    .expect("file could not be written :(");
                return Ok(());
            }
        }
        Err(Error::Generic(
            "Something happend while parsing the config".to_string(),
        ))
    }
}

pub fn load_config() -> Result<Config, String> {
    if !get_full_config_file_path()?.is_file() {
        match fs::create_dir_all(get_full_config_path()?) {
            Ok(_) => {}
            Err(e) => return Err(format!("{:?}", e)),
        }
        let config = Config::default_config()?;
        config.save()?;
        return Ok(config);
    }
    let json = match fs::read_to_string(get_full_config_file_path()?) {
        Ok(o) => o,
        Err(e) => return Err(format!("{:?}", e)),
    };

    let config = match serde_json::from_str(&json) {
        Ok(o) => o,
        Err(e) => return Err(format!("{:?}", e)),
    };
    Ok(config)
}

/// /home/$USER/.store
pub fn get_default_store_path() -> Result<PathBuf, String> {
    if let Some(home_dir) = home_dir() {
        return Ok(home_dir.join(".store"));
    }
    Err("home dir was not found".to_string())
}
/// /home/$USER/.config/bloi
pub fn get_full_config_path() -> Result<PathBuf, String> {
    if let Some(config_dir) = config_dir() {
        return Ok(config_dir.join("bloi"));
    }
    Err("config dir was not found".to_string())
}
/// /home/$USER/.config/bloi/config.json
pub fn get_full_config_file_path() -> Result<PathBuf, String> {
    Ok(get_full_config_path()?.join("config.json"))
}
