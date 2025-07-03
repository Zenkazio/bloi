use std::env;
use std::path::PathBuf;

use bloi::store_routine;

use crate::cli::*;
use crate::config::{Config, get_default_store_path};
use crate::error::*;
use crate::git::*;

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
            println!("Added {:?} to managed files", path);
            println!("It will be included in the next store operation");
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
            let target_path = current_dir.join(path);
            if config.adds.remove(&target_path) {
                config.save()?;
                println!("Removed {:?} from managed files", target_path);
                list_adds(&config);
                println!("Original content has been restored");
                println!("unstoring at the moment to dangerous");
                //unstore_routine(&target_path, &get_default_store_path()?);
            } else {
                println!("{:?} was not found in the config", target_path);
            }
        }
        Some(("change-store-dir", _)) => {
            todo!("currently not possible");
        }
        Some(("list", _)) => {
            println!("Currently managed files and directories:");
            // If list is empty
            if config.adds.is_empty() {
                println!("  No files are currently being managed.");
                println!("  Use 'bloi add <path>' to start managing files.");
            }
            list_adds(&config);
        }
        Some(("store", _)) => {
            pre_store()?;
            config = config::load_config()?;
            println!("Starting storage operation for all managed files...");
            store(&config)?;
            println!("Storage completed successfully. All files are now symlinked.");
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
