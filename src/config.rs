use bloi::*;
use std::collections::HashSet;
use std::{fs, io::Write, path::PathBuf};

use dirs::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    adds: HashSet<PathBuf>,
    use_git: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            adds: HashSet::new(),
            use_git: false,
        }
    }
}
impl Config {
    pub fn load_config() -> Result<Config> {
        if !get_full_config_file_path()?.is_file() {
            fs::create_dir_all(get_default_store_path()?)?;
            let config = Config::default();
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
    pub fn list_adds(&self) {
        for entry in self.get_adds() {
            println!("- {:?}", entry);
        }
        println!();
    }
    pub fn get_adds(&self) -> &HashSet<PathBuf> {
        &self.adds
    }
    pub fn change_adds(&mut self) -> &mut HashSet<PathBuf> {
        &mut self.adds
    }
    pub fn get_use_git(&self) -> &bool {
        &self.use_git
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
/// /home/$USER/.store/config.json
fn get_full_config_file_path() -> Result<PathBuf> {
    Ok(get_default_store_path()?.join("config.json"))
}
