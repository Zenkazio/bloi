#![allow(unused)]

use std::env;
use std::path::PathBuf;

use crate::cli::*;
use crate::config::{Config, get_default_store_path};
use crate::error::*;
use crate::git::*;
use bloi::store_routine;

mod cli;
mod config;
mod error;
mod git;

fn main() -> Result<()> {
    let mut config = config::load_config()?;

    match build_cli().get_matches().subcommand() {
        Some(("add", sub_m)) => {
            let path = match sub_m.get_one::<PathBuf>("path") {
                Some(s) => s,
                None => {
                    return Err(Error::UnconventionalClapArgMissing);
                }
            };
            let current_dir = env::current_dir().map_err(Error::Io)?;
            config.adds.insert(current_dir.join(path));
            config.save()?;
            println!("added {:?}", path);
            list_adds(&config);
        }
        Some(("remove", sub_m)) => {
            let path = match sub_m.get_one::<PathBuf>("path") {
                Some(s) => s,
                None => {
                    return Err(Error::UnconventionalClapArgMissing);
                }
            };
            let current_dir = env::current_dir().map_err(Error::Io)?;
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

fn store(config: &Config) -> Result<()> {
    for target_path in &config.adds {
        store_routine(target_path, &get_default_store_path()?);
    }
    Ok(())
}

fn list_adds(config: &Config) {
    for entry in &config.adds {
        println!("- {:?}", entry);
    }
    println!();
}

fn pre_store() -> Result<()> {
    git_detect_potential_conflict(&get_default_store_path()?)?;
    git_pull(&get_default_store_path()?)?;
    Ok(())
}

fn post_store() -> Result<()> {
    git_add_all(&get_default_store_path()?)?;
    git_commit_with_date(&get_default_store_path()?)?;
    git_push(&get_default_store_path()?)?;
    Ok(())
}
