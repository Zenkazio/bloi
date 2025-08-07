use crate::cli::*;
use crate::config::*;
use crate::git::*;
use bloi::*;
use clap::Parser;
use std::env;

mod cli;
mod config;
mod git;

fn main() -> Result<()> {
    let mut config = Config::load_config()?;

    if config.use_git {
        git_init(&get_default_store_path()?)?;
    }

    match Cli::parse().command {
        Commands::Add { target_path } => {
            let current_dir = env::current_dir()?;
            let temp_path = current_dir.join(target_path);
            println!("Added {:?} to managed files", &temp_path);
            config.files.push(temp_path);
            config.save()?;
            println!("It will be included in the next store operation");
            config.list_files();
        }
        Commands::Remove { pos } => {
            let t = config.files.remove(pos - 1);
            config.save()?;
            println!("Removed {:?} from managed files", t);
            config.list_files();
            // unstore_routine(pos - 1, &get_default_store_path()?)?;
            // println!("Original content has been restored");
        }
        Commands::ChangeStoreDir { store_path } => {
            println!("Change store dir: {:?}", store_path);
            config.store_dir = store_path;
            config.save()?;
        }
        Commands::List => {
            println!("Currently managed files and directories:");
            // If list is empty
            if config.files.is_empty() {
                println!("  No files are currently being managed.");
                println!("  Use 'bloi add' to start managing files.");
            }
            config.list_files();
        }
        Commands::Store => {
            pre_store(&config)?;
            config = Config::load_config()?;

            println!("Starting storage operation for all managed files...");
            store(&config)?;
            println!("Storage completed successfully. All files are now symlinked.");

            post_store(&config)?;
        }
        Commands::Git => {
            config.switch_git();
            config.save()?;
        }
    }
    Ok(())
}

fn store(config: &Config) -> Result<()> {
    for target in &config.files {
        store_routine(target, &config.store_dir)?;
    }
    Ok(())
}

fn pre_store(config: &Config) -> Result<()> {
    if config.use_git {
        git_detect_potential_conflict(&get_default_store_path()?)?;
        git_pull(&get_default_store_path()?)?;
    }
    Ok(())
}

fn post_store(config: &Config) -> Result<()> {
    if config.use_git {
        git_add_all(&get_default_store_path()?)?;
        git_commit_with_date(&get_default_store_path()?)?;
        git_push(&get_default_store_path()?)?;
    }
    Ok(())
}
