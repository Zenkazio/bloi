//#![allow(unused)]

use std::env;
use std::path::PathBuf;

use crate::cli::*;
use crate::config::{Config, get_default_store_path};
use crate::git::{
    detect_potential_conflict, git_add_all, git_commit_with_date, git_fetch, git_pull, git_push,
};
use crate::utils::{UserChoice, store_routine};
mod cli;
mod config;
mod git;
mod utils;

fn main() -> Result<(), String> {
    config::load_config()?;

    mv!(git_add_all(&get_default_store_path()?));
    mv!(git_commit_with_date(&get_default_store_path()?));
    mv!(git_fetch(&get_default_store_path()?));
    detect_potential_conflict(&get_default_store_path()?)?;
    mv!(git_pull(&get_default_store_path()?));

    let mut config = config::load_config()?;

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
    mv!(git_add_all(&get_default_store_path()?));
    mv!(git_commit_with_date(&get_default_store_path()?));
    mv!(git_push(&get_default_store_path()?));
    Ok(())
}

// fn unstore(config: &Config) -> Result<(), String> {
//     for _path in &config.adds {}
//     Ok(())
// }
