use std::{path::PathBuf, process::Command};

pub fn git_add_all(path: &PathBuf) {
    Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .status()
        .unwrap();
}
