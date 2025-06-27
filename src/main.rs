#![allow(unused)]

use clap::Parser;

use crate::cli::*;
use crate::prelude::*;
mod cli;
mod config;
mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let mut config = config::load_config()?;

    utils::check_store_dir(&config.store_dir);

    let matches = build_cli().get_matches();
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let path = sub_m.get_one::<std::path::PathBuf>("path").unwrap().clone();
            config.adds.push(path);
            config.save();
        }
        Some(("store", _)) => {
            for path in &config.adds {
                match utils::process_path_based_on_type(path, &config.store_dir) {
                    Ok(_) => {}
                    Err(e) => {
                        dbg!(e);
                    }
                };
            }
        }
        _ => {}
    }
    dbg!(config);

    Ok(())
}
