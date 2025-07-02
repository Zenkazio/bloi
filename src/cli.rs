use clap::{Arg, Command};
use std::path::PathBuf;

pub fn build_cli() -> Command {
    Command::new("bloi")
        .version("0.2")
        .about("Manage files by storing them centrally and replacing with symlinks")
        .after_help("Tip: Use `bloi help <command>` for more details on a specific subcommand.")
        .subcommand(
            Command::new("add")
                .about("Add a file or directory to be managed by bloi")
                .arg(
                    Arg::new("path")
                        .help("Path to the file/directory you want to manage (will be stored as absolute path)")
                        .required(true)
                        .value_parser(validate_path)
                        .value_name("PATH")
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Stop managing a file or directory and restore original content")
                .arg(
                    Arg::new("path")
                        .help("The file or directory path to be removed from the store (absolute path stored)")
                        .required(true)
                        .value_parser(validate_path_no_exist)
                        .value_name("PATH")
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("change-store-dir")
                .about("Change the directory where files are stored (default: ~/.store)")
                .arg(
                    Arg::new("path")
                        .help("path to store dir")
                        .required(true)
                        .value_parser(validate_path_no_exist)
                        .value_name("PATH")
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List all files and directories currently managed by bloi"),
        )
        .subcommand(
            Command::new("store")
                .about("Copy managed files to storage and replace originals with symlinks"),
        )
}

fn validate_path(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() {
        Ok(path)
    } else {
        Err(String::from("path doesn't exist"))
    }
}

fn validate_path_no_exist(s: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(s))
}
