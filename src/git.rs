use std::{path::PathBuf, process::Command};

pub fn git_add_all(path: &PathBuf) {
    Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .status()
        .unwrap();
}
pub fn git_commit_with_date(path: &PathBuf) {
    let date = Command::new("date").output().unwrap();
    let msg = String::from_utf8_lossy(&date.stdout).trim().to_string();

    Command::new("git")
        .args(["commit", "-m", &msg])
        .current_dir(path)
        .status()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, process::Command};

    use crate::git::{git_add_all, git_commit_with_date};

    #[test]
    fn test_commands() {
        dbg!(Command::new("echo").arg("hallo").status().unwrap());

        let base = PathBuf::from("/home/zenkazio/Projects/bloi/");
        git_add_all(&base);
        git_commit_with_date(&base);
    }
}
