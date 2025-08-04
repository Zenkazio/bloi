use bloi::*;
use std::{
    collections::HashSet,
    path::PathBuf,
    process::{Command, Output},
};

fn run_git_command(args: &[&str], path: &PathBuf) -> Result<Output> {
    Command::new("git")
        .args(args)
        .current_dir(path)
        .output()
        .map_err(Error::Io)
}
pub fn git_init(path: &PathBuf) -> Result<()> {
    if !path.join(".git").exists() {
        println!("Initializing Git repository in directory: {:?}", path);
        run_git_command(&["init"], path)?;
    }
    Ok(())
}
pub fn git_add_all(path: &PathBuf) -> Result<Output> {
    println!("Adding all files in {:?} to git staging area", path);
    run_git_command(&["add", "."], path)
}
pub fn git_commit_with_date(path: &PathBuf) -> Result<Output> {
    let date = Command::new("date").output().map_err(Error::Io)?;
    let date_as_string = String::from_utf8_lossy(&date.stdout).trim().to_string();
    let mut msg = String::from("Automatic backup created by bloi");
    msg.push_str("\n");
    msg.push_str(&date_as_string);

    println!("Committing changes to storage repository...");
    run_git_command(&["commit", "-m", &msg], path)
}
pub fn git_fetch(path: &PathBuf) -> Result<Output> {
    println!("Perform \"git fetch\" in {:?}", path);
    run_git_command(&["fetch"], path)
}
pub fn git_push(path: &PathBuf) -> Result<Output> {
    println!("Perform \"git push\" in {:?}", path);
    run_git_command(&["push"], path)
}
pub fn git_pull(path: &PathBuf) -> Result<Output> {
    println!("Perform \"git pull\" in {:?}", path);
    run_git_command(&["pull"], path)
}
fn get_changed_files(args: &[&str], path: &PathBuf) -> Result<HashSet<String>> {
    let output = run_git_command(args, path)?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|x| x.to_string())
        .collect())
}

pub fn git_detect_potential_conflict(path: &PathBuf) -> Result<()> {
    git_add_all(path)?;
    git_commit_with_date(path)?;
    git_fetch(path)?;

    println!("Perform git conflict detection");
    let local_changes = get_changed_files(&["diff", "--name-only", "HEAD"], path)?;
    let remote_changes = get_changed_files(&["diff", "--name-only", "HEAD", "origin/main"], path)?;

    let conflicts: Vec<_> = local_changes.intersection(&remote_changes).collect();
    if !conflicts.is_empty() {
        println!("Potential merge conflicts detected in the following files:");
        for file in conflicts {
            println!(" - {}", file);
        }
        println!("No automatic merge performed - must be done by user");
        return Err(Error::GitPotentialConflict);
    }
    Ok(())
}
