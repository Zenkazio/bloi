#![allow(unused)]

use crate::error::*;
use std::env;
use std::path::PathBuf;

use crate::cli::*;
use crate::config::{Config, get_default_store_path};
use crate::git::*;

mod cli;
mod config;
mod error;
mod git;

fn main() -> Result<()> {
    let mut config = config::load_config().unwrap();

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
            println!("added {:?}", path);
            list_adds(&config);
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
            println!("removed {:?}", path);
            list_adds(&config);
        }
        Some(("change-store-dir", _)) => {
            todo!("currently not possible");
        }
        Some(("list", _)) => {
            list_adds(&config);
        }
        Some(("store", _)) => {
            pre_store()?;
            config = config::load_config()?;
            store(&config)?;
            post_store()?;
        }
        _ => {}
    }
    Ok(())
}

fn store(config: &Config) -> Result<(), String> {
    for target_path in &config.adds {
        let mut user_choice = UserChoice::NoChoice;
        match store_routine(target_path, &mut user_choice) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e)
            }
        }
    }
    Ok(())
}

fn list_adds(config: &Config) {
    for entry in &config.adds {
        println!("- {:?}", entry);
    }
    println!();
}

fn pre_store() -> Result<(), String> {
    mv!(git_add_all(&get_default_store_path()?));
    mv!(git_commit_with_date(&get_default_store_path()?));
    mv!(git_fetch(&get_default_store_path()?));
    git_detect_potential_conflict(&get_default_store_path()?)?;
    mv!(git_pull(&get_default_store_path()?));
    Ok(())
}

fn post_store() -> Result<(), String> {
    mv!(git_add_all(&get_default_store_path()?));
    mv!(git_commit_with_date(&get_default_store_path()?));
    mv!(git_push(&get_default_store_path()?));
    Ok(())
}
