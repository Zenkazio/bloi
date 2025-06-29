//#![allow(unused)]

use std::path::PathBuf;
use std::{env, fs};

use crate::cli::*;
use crate::config::Config;
use crate::git::git_add_all;
use crate::utils::{UserChoice, store_routine};
mod cli;
mod config;
mod git;
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
            println!("{:?}", config.adds);
        }
        Some(("remove", sub_m)) => {
            let path = match sub_m.get_one::<PathBuf>("path") {
                Some(s) => s,
                None => {
                    return Err(format!("could not parse path to PathBuf lol\n{:?}", sub_m));
                }
            };
            let current_dir = mv!(env::current_dir());
            config.adds.remove(&current_dir.join(path));
            config.save()?;
            println!("{:?}", config.adds);
        }
        Some(("change-store-dir", _)) => {
            todo!("do it in the config.json until then");
        }
        Some(("list", _)) => {
            println!("{:?}", config.adds);
        }
        Some(("store", _)) => store(&config)?,
        _ => {}
    }
    Ok(())
}

//fn store_dir_help() {}

fn store(config: &Config) -> Result<(), String> {
    for target_path in &config.adds {
        let mut user_choice = UserChoice::NoChoice;
        match store_routine(target_path, config, &mut user_choice) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e)
            }
        }
    }
    Ok(())
}

// fn unstore(config: &Config) -> Result<(), String> {
//     for _path in &config.adds {}
//     Ok(())
// }
