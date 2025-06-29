use std::{
    collections::HashSet,
    path::PathBuf,
    process::{Command, Output},
};

use crate::mv;
fn run_git_command(args: &[&str], path: &PathBuf) -> std::io::Result<Output> {
    Command::new("git").args(args).current_dir(path).output()
}

pub fn git_add_all(path: &PathBuf) -> std::io::Result<Output> {
    run_git_command(&["add", "."], path)
}
pub fn git_commit_with_date(path: &PathBuf) -> std::io::Result<Output> {
    let date = Command::new("date").output()?;
    let msg = String::from_utf8_lossy(&date.stdout).trim().to_string();

    run_git_command(&["commit", "-m", &msg], path)
}
pub fn git_fetch(path: &PathBuf) -> std::io::Result<Output> {
    run_git_command(&["fetch"], path)
}

fn get_changed_files(args: &[&str], path: &PathBuf) -> std::io::Result<HashSet<String>> {
    let output = run_git_command(args, path)?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|x| x.to_string())
        .collect())
}

pub fn detect_potential_conflict(path: &PathBuf) -> Result<(), String> {
    let local_changes = mv!(get_changed_files(&["diff", "--name-only", "HEAD"], path));
    let remote_changes = mv!(get_changed_files(
        &["diff", "--name-only", "HEAD", "origin/main"],
        path
    ));

    let conflicts: Vec<_> = local_changes.intersection(&remote_changes).collect();
    if !conflicts.is_empty() {
        println!("Potential merge conflicts detected in the following files:");
        for file in conflicts {
            println!(" - {}", file);
        }
        return Err(
            "Please resolve conflicts manually. No automatic merge will be performed.".to_string(),
        );
    }
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use std::{path::PathBuf, process::Command};

//     use crate::git::{git_add_all, git_commit_with_date};

//     #[test]
//     fn test_commands() {
//         dbg!(Command::new("echo").arg("hallo").status().unwrap());

//         // let base = PathBuf::from("/home/zenkazio/Projects/bloi/");
//         // git_add_all(&base);
//         // git_commit_with_date(&base);
//     }
// }
