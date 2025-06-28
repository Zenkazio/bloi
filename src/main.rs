//#![allow(unused)]

use std::env;
use std::path::PathBuf;

use crate::cli::*;
mod cli;
mod config;
mod utils;

fn main() -> Result<(), String> {
    let config = config::load_config()?;

    //utils::create_dir(&config.store_path)?;
    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    let matches = build_cli().get_matches();
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let path = sub_m.get_one::<std::path::PathBuf>("path").unwrap().clone();
            config.adds.insert(current_dir.join(path));
            config.save()?;
        }
        Some(("rm", sub_m)) => {
            let path = sub_m.get_one::<std::path::PathBuf>("path").unwrap().clone();
            config.adds.remove(&current_dir.join(path));
            config.save()?;
        }
        Some(("list", _)) => {
            let path = sub_m.get_one::<std::path::PathBuf>("path").unwrap().clone();
            config.adds.remove(&current_dir.join(path));
            config.save()?;
        }
        Some(("store", _)) => {
            for path in &config.adds {
                match utils::decide_state_and_proccess_path(&path, &config.store_path, None) {
                    Ok(_) => {}
                    Err(e) => match e {
                        Error::Generic(s) => {
                            println!("{}", s);
                        }
                    },
                };
            }
        }
        _ => {}
    }
    Err("halllo".to_string())
}
