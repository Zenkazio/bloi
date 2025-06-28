//#![allow(unused)]

use std::path::PathBuf;
use std::{env, fs};

use crate::cli::*;
use crate::config::Config;
use crate::utils::store_routine;
mod cli;
mod config;
mod utils;

fn main() -> Result<(), String> {
    let mut config = config::load_config()?;

    //utils::create_dir(&config.store_path)?;
    if !config.store_path.exists() {
        match fs::create_dir_all(&config.store_path) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        }
    }

    match build_cli().get_matches().subcommand() {
        Some(("add", sub_m)) => {
            let path = match sub_m.get_one::<PathBuf>("path") {
                Some(s) => s,
                None => {
                    return Err(format!("could not parse path to PathBuf lol\n{:?}", sub_m));
                }
            };
            let current_dir = mv!(env::current_dir());
            config.adds.insert(current_dir.join(path));
            config.save()?;
        }
        Some(("rm", sub_m)) => {
            let path = match sub_m.get_one::<PathBuf>("path") {
                Some(s) => s,
                None => {
                    return Err(format!("could not parse path to PathBuf lol\n{:?}", sub_m));
                }
            };
            let current_dir = mv!(env::current_dir());
            config.adds.remove(&current_dir.join(path));
            config.save()?;
        }
        Some(("list", _)) => {
            println!("{:?}", config.adds)
        }
        Some(("store", _)) => store(&config)?,
        _ => {}
    }
    Ok(())
}

fn store_dir_help() {}

fn store(config: &Config) -> Result<(), String> {
    for target_path in &config.adds {
        match store_routine(target_path, config) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e)
            }
        }
    }
    Ok(())
}

fn unstore(config: &Config) -> Result<(), String> {
    for _path in &config.adds {}
    Ok(())
}
