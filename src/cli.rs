use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "bloi",
    version = "0.2",
    about = "Manage files by storing them centrally and replacing with symlinks",
    after_help = "Tip: Use `bloi help <command>` for more details on a specific subcommand."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a file or directory to be managed by bloi
    Add {
        #[arg(
            help = "Path to the file/directory you want to manage",
            value_parser = validate_path_no_exist
        )]
        target_path: PathBuf,
        #[arg(
            value_parser = validate_path_no_exist
        )]
        path_in_store: PathBuf,
    },
    /// Stop managing a file or directory and restore original content
    Remove {
        #[arg(
            help = "the position in list to remove",
            value_parser = validate_path_no_exist
        )]
        pos: usize,
    },
    /// Change the directory where files are stored (default: ~/.store)
    ChangeStoreDir {
        #[arg(
            help = "Path to new store directory",
            value_parser = validate_path_no_exist
        )]
        store_path: PathBuf,
    },
    /// List all files and directories currently managed by bloi
    List,
    /// Copy managed files to storage and replace originals with symlinks
    Store,
    /// Toggle (on/off) git integration (needs remote repo at the moment)
    Git,
}

// fn validate_path(s: &str) -> Result<PathBuf, String> {
//     let path = PathBuf::from(s);
//     if path.exists() {
//         Ok(path)
//     } else {
//         Err("path doesn't exist".into())
//     }
// }

fn validate_path_no_exist(s: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(s))
}
