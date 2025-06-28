use clap::{Arg, Command};
use std::path::PathBuf;

pub fn build_cli() -> Command {
    Command::new("bloi")
        .version("0.1")
        .about("Program stores files and directories and replaces them with a (soft) symlink")
        .after_help("Tip: Use `bloi help <command>` for more details on a specific subcommand.")
        .subcommand(
            Command::new("add")
                .about("Adds file or directory to the store")
                .arg(
                    Arg::new("path")
                        .help("The file or directory path to be added to the store (absolute path stored)")
                        .required(true)
                        .value_parser(validate_path)
                        .value_name("PATH")
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("remove file or directory from storeing process")
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
                .about("change store dir default /home/$USER/.store")
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
                .about("lists stored files/dir"),
        )
        .subcommand(
            Command::new("store")
                .about("Copies file (recursively) and replaces them with symlinks"),
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
