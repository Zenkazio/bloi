use crate::error::*;
use std::collections::HashSet;
use std::{fs, io::Write, path::PathBuf};

use dirs::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub adds: HashSet<PathBuf>,
}

impl Config {
    fn default_config() -> Self {
        let temp = Self {
            adds: HashSet::new(),
        };
        temp
    }
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self).map_err(Error::SerdeJson)?;
        let mut file = fs::File::create(get_full_config_file_path()?).map_err(Error::Io)?;
        file.write_all(json.as_bytes());
        Ok(())
    }
}

pub fn load_config() -> Result<Config> {
    if !get_full_config_file_path()?.is_file() {
        fs::create_dir_all(get_default_store_path()?);
        let config = Config::default_config();
        config.save()?;
        return Ok(config);
    }
    let json = fs::read_to_string(get_full_config_file_path()?).map_err(Error::Io)?;

    let config = serde_json::from_str(&json).map_err(Error::SerdeJson)?;
    Ok(config)
}

/// /home/$USER/.store/
pub fn get_default_store_path() -> Result<PathBuf> {
    if let Some(home_dir) = home_dir() {
        return Ok(home_dir.join(".store/"));
    }
    Err(Error::HomeDirNotFound)
}
/// /home/$USER/.store/config.json
pub fn get_full_config_file_path() -> Result<PathBuf> {
    Ok(get_default_store_path()?.join("config.json"))
}
