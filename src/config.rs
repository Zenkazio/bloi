use bloi::*;
use std::{fs, io::Write, path::PathBuf};

use dirs::{config_dir, home_dir};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub store_dir: PathBuf,
    pub files: Vec<(PathBuf, PathBuf)>,
    pub use_git: bool,
}

impl Config {
    pub fn load_config() -> Result<Config> {
        if !get_full_config_file_path()?.is_file() {
            make_dir_all_file(&get_full_config_file_path()?)?;
            let config = Config {
                files: Vec::new(),
                use_git: false,
                store_dir: get_default_store_path()?,
            };
            config.save()?;
            return Ok(config);
        }
        let json = fs::read_to_string(get_full_config_file_path()?)?;
        let config = serde_json::from_str(&json)?;
        Ok(config)
    }
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        let mut file = fs::File::create(get_full_config_file_path()?)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
    pub fn list_files(&self) {
        for (u, (target, store)) in self.files.iter().enumerate() {
            let pos = u + 1;
            let target_mod = self.store_dir.join(target);
            let store_mode = self.store_dir.join(store);
            println!("{pos}: {target_mod:?} <-> {store_mode:?}");
        }
        println!();
    }
    pub fn switch_git(&mut self) {
        self.use_git = !self.use_git
    }
}

/// /home/$USER/.store/
pub fn get_default_store_path() -> Result<PathBuf> {
    if let Some(home_dir) = home_dir() {
        return Ok(home_dir.join(".store/"));
    }
    Err(Error::HomeDirNotFound)
}

/// /home/$USER/.config/bloi/config.json
fn get_full_config_file_path() -> Result<PathBuf> {
    if let Some(config_dir) = config_dir() {
        return Ok(config_dir.join("bloi/config.json"));
    }
    Err(Error::ConfigDirNotFound)
}
