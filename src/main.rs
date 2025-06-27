#![allow(unused)]
mod cli;
mod config;
mod error;
mod prelude;

use crate::prelude::*;

fn main() -> Result<()> {
    let mut config = config::load_config();
    dbg!(config);
    Ok(())
}
