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

// #[cfg(test)]
// mod tests {
//     use std::{path::PathBuf, process::Command};

//     use crate::git::{
//         detect_potential_conflict, git_add_all, git_commit_with_date, git_fetch, git_pull, git_push,
//     };

//     #[test]
//     fn test_commands() {
//         dbg!(Command::new("echo").arg("hallo").status().unwrap());

// let base = PathBuf::from("/home/zenkazio/Projects/bloi/");
// git_add_all(&base);
// git_commit_with_date(&base);
//   }
// #[test]
// fn test_git_commands_working() {
//     //this is manily used just to automate git with this project...
//     let path = &PathBuf::from("/home/zenkazio/Projects/bloi/");

//     git_add_all(path).unwrap();
//     git_commit_with_date(path).unwrap();
//     git_fetch(path).unwrap();
//     detect_potential_conflict(path).unwrap();
//     git_pull(path).unwrap();
//     git_commit_with_date(path).unwrap();
//     git_push(path).unwrap();

//     Command::new("cargo").args(&["build", "--release"]);
// }
//}

//         // let base = PathBuf::from("/home/zenkazio/Projects/bloi/");
//         // git_add_all(&base);
//         // git_commit_with_date(&base);
//     }
//     #[test]
//     fn test_git_commands_working() {
//         //this is manily used just to automate git with this project...
//         let path = &PathBuf::from("/home/zenkazio/Projects/bloi/");

//         git_add_all(path).unwrap();
//         git_commit_with_date(path).unwrap();
//         git_fetch(path).unwrap();
//         detect_potential_conflict(path).unwrap();
//         git_pull(path).unwrap();
//         git_commit_with_date(path).unwrap();
//         git_push(path).unwrap();

//         Command::new("cargo").args(&["build", "--release"]);
//     }
// }
