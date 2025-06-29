use std::collections::HashSet;
use std::{fs, io::Write, path::PathBuf};

use dirs::home_dir;
use serde::{Deserialize, Serialize};

use crate::mv;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub adds: HashSet<PathBuf>,
}

impl Config {
    pub fn default_config() -> Result<Self, String> {
        let temp = Self {
            adds: HashSet::new(),
        };
        //temp.adds.insert(get_full_config_path()?);
        Ok(temp)
    }
    pub fn save(&self) -> Result<(), String> {
        let json = match serde_json::to_string_pretty(&self) {
            Ok(o) => o,
            Err(e) => return Err(format!("{:?}", e)),
        };
        let mut file = match fs::File::create(get_full_config_file_path()?) {
            Ok(o) => o,
            Err(e) => return Err(e.to_string()),
        };
        match file.write_all(json.as_bytes()) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        }
        Ok(())
    }
}

pub fn load_config() -> Result<Config, String> {
    if !get_full_config_file_path()?.is_file() {
        mv!(fs::create_dir_all(get_default_store_path()?));
        let config = Config::default_config()?;
        config.save()?;
        return Ok(config);
    }
    let json = mv!(fs::read_to_string(get_full_config_file_path()?));

    let config = mv!(serde_json::from_str(&json));
    Ok(config)
}

/// /home/$USER/.store
pub fn get_default_store_path() -> Result<PathBuf, String> {
    if let Some(home_dir) = home_dir() {
        return Ok(home_dir.join(".store"));
    }
    Err("home dir was not found".to_string())
}
/// /home/$USER/.store/config.json
pub fn get_full_config_file_path() -> Result<PathBuf, String> {
    Ok(get_default_store_path()?.join("config.json"))
}
