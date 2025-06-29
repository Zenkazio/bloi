use std::{path::PathBuf, process::Command};

pub fn git_add_all(path: &PathBuf) {
    Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .status()
        .unwrap();
}
pub fn git_commit_with_date(path: &PathBuf) {
    dbg!(
        Command::new("git")
            .args(["commit", "-m", "$(date)"])
            .current_dir(path)
            .status()
            .unwrap()
    );
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, process::Command};

    use crate::git::git_commit_with_date;

    #[test]
    fn test_commands() {
        dbg!(Command::new("echo").arg("hallo").status().unwrap());

        dbg!(
            Command::new("git")
                .args(["add", "."])
                .current_dir("/home/zenkazio/Projects/bloi/")
                .status()
                .unwrap()
        );
        git_commit_with_date(&PathBuf::from("/home/zenkazio/Projects/bloi/"));
    }
}
