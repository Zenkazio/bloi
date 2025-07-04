use std::env;
use std::path::PathBuf;

use crate::cli::*;
use crate::config::*;
use crate::git::*;
use bloi::*;

mod cli;
mod config;
mod git;

fn main() -> Result<()> {
    let mut config = Config::load_config()?;

    if *config.get_use_git() {
        git_init(&get_default_store_path()?)?;
    }

    match build_cli().get_matches().subcommand() {
        Some(("add", sub_m)) => {
            let path = match sub_m.get_one::<PathBuf>("path") {
                Some(s) => s,
                None => {
                    return Err(Error::UnconventionalClapArgMissing("path".to_string()));
                }
            };
            let current_dir = env::current_dir().map_err(Error::Io)?;
            config.change_adds().insert(current_dir.join(path));
            config.save()?;
            println!("Added {:?} to managed files", path);
            println!("It will be included in the next store operation");
            config.list_adds();
        }
        Some(("remove", sub_m)) => {
            let path = match sub_m.get_one::<PathBuf>("path") {
                Some(s) => s,
                None => {
                    return Err(Error::UnconventionalClapArgMissing("path".to_string()));
                }
            };
            let current_dir = env::current_dir().map_err(Error::Io)?;
            let target_path = current_dir.join(path);
            if config.change_adds().remove(&target_path) {
                config.save()?;
                println!("Removed {:?} from managed files", target_path);
                config.list_adds();
                unstore_routine(&target_path, &get_default_store_path()?)?;
                println!("Original content has been restored");
            } else {
                println!("{:?} was not found in the config", target_path);
            }
        }
        Some(("change-store-dir", _)) => {
            todo!("currently not possible");
        }
        Some(("git", _)) => {
            config.switch_git();
            config.save()?;
        }
        Some(("list", _)) => {
            println!("Currently managed files and directories:");
            // If list is empty
            if config.get_adds().is_empty() {
                println!("  No files are currently being managed.");
                println!("  Use 'bloi add <path>' to start managing files.");
            }
            config.list_adds();
        }
        Some(("store", _)) => {
            pre_store(&config)?;
            config = Config::load_config()?;
            println!("Starting storage operation for all managed files...");
            store(&config)?;
            println!("Storage completed successfully. All files are now symlinked.");
            post_store(&config)?;
        }
        _ => {}
    }
    Ok(())
}

fn store(config: &Config) -> Result<()> {
    for target_path in config.get_adds() {
        store_routine(target_path, &get_default_store_path()?)?;
    }
    Ok(())
}

fn pre_store(config: &Config) -> Result<()> {
    if *config.get_use_git() {
        git_detect_potential_conflict(&get_default_store_path()?)?;
        git_pull(&get_default_store_path()?)?;
    }
    Ok(())
}

fn post_store(config: &Config) -> Result<()> {
    if *config.get_use_git() {
        git_add_all(&get_default_store_path()?)?;
        git_commit_with_date(&get_default_store_path()?)?;
        git_push(&get_default_store_path()?)?;
    }
    Ok(())
}
