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
    pub fn default_config() -> Result<Self> {
        let mut temp = Self {
            store_path: get_store_path()?,
            adds: HashSet::new(),
        };
        temp.adds.insert(get_full_config_path()?);
        Ok(temp)
    }
    pub fn save(&self) -> Result<()> {
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

pub fn load_config() {
    if let Ok(json) = fs::read_to_string(get_full_config_file_path()?) {
        if let Ok(config) = serde_json::from_str(&json) {
            return Ok(config);
        }
    }
    Ok(create_config()?)
}

pub fn create_config() {
    if let Some(dir) = get_full_config_file_path()?.parent() {
        utils::create_dir(&dir.to_path_buf())?;
    }
    Config::default_config()?.save()?;
    Config::default_config()
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

#[cfg(test)]
mod tests {

    #[test]
    fn check_path_building() {}
}
