use std::{fs, io::stdin, os::unix::fs::symlink, path::PathBuf};

use crate::{
    config::{Config, get_default_store_path},
    path_tree::PathTreeEqualizer,
};

#[macro_export]
macro_rules! mv {
    //match and get value
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return Err(e.to_string()),
        }
    };
}

#[derive(Debug)]
enum PathState {
    Symlink,
    NoExist,
    File,
    Dir,
}

pub fn store_routine(
    target_path: &PathBuf,
    config: &Config,
    user_choice: &mut UserChoice,
) -> Result<(), String> {
    let path_to_store: PathBuf = get_default_store_path()?.join(absolute_to_relative(target_path));
    let mut target_state = classify_path(target_path);
    let mut store_state = classify_path(&path_to_store);
    // dbg!(target_path);
    // dbg!(&path_to_store);
    let path_tree_eq =
        PathTreeEqualizer::from_paths(path_to_store.clone(), target_path.clone(), true);

    loop {
        match (target_state, store_state) {
            (_, PathState::Symlink) => {
                return Err(format!(
                    "serious problem symlink in store doesn't make sense\n{:?}",
                    path_to_store
                ));
            }
            (PathState::Symlink, PathState::File) => {} //nothing to do here this is the wanted state for storing
            (PathState::Symlink, _) => {
                return Err(format!(
                    "this is a hard problem symlink exists but nothing in store serious data loss possible\n{:?}",
                    target_path
                ));
            }
            (PathState::File, PathState::File) => {
                match *user_choice {
                    UserChoice::NoChoice | UserChoice::TakeStore | UserChoice::TakeTarget => {
                        println!("{:?}", target_path);
                        get_user_choice(user_choice)?;
                    }
                    _ => {} //skip it
                }
                match *user_choice {
                    UserChoice::TakeStore | UserChoice::TakeStoreAll => {
                        mv!(fs::remove_file(target_path));
                        target_state = PathState::NoExist;
                        store_state = PathState::File;
                        continue;
                    }
                    UserChoice::TakeTarget | UserChoice::TakeTargetAll => {
                        mv!(fs::remove_file(&path_to_store));
                        target_state = PathState::File;
                        store_state = PathState::NoExist;
                        continue;
                    }
                    _ => {
                        return Err(format!("this should not be possible"));
                    }
                }

                //return Err(format!("conflict both are files\n{:?}", target_path));
            } //conflict which user needs to resolve
            (PathState::File, PathState::NoExist) => {
                mv!(fs::create_dir_all(path_to_store.parent().unwrap()));
                mv!(fs::rename(target_path, &path_to_store));
                mv!(symlink(&path_to_store, target_path));
            } // this is the case for storing
            (PathState::NoExist, PathState::NoExist) => {
                return Err(format!(
                    "nothing in target and store mabye old path in config\n{:?}",
                    target_path
                ));
            }
            (PathState::Dir, PathState::NoExist) => {
                mv!(fs::create_dir_all(&path_to_store));
                for entry in mv!(fs::read_dir(target_path)) {
                    store_routine(&mv!(entry).path(), config, user_choice)?;
                }
            }
            (PathState::NoExist, PathState::File) => {
                mv!(fs::create_dir_all(target_path.parent().unwrap()));
                mv!(symlink(&path_to_store, target_path));
            } // this is the case if no target exists just create symlink
            (PathState::Dir, PathState::File) => {
                return Err(format!(
                    "serious problem dir != file mismatch\n{:?}",
                    target_path
                ));
            }
            (PathState::File, PathState::Dir) => {
                return Err(format!(
                    "serious problem dir != file mismatch\n{:?}",
                    target_path
                ));
            }
            (PathState::Dir, PathState::Dir) => {
                for entry in mv!(fs::read_dir(target_path)) {
                    //dbg!(mv!(&entry).path());
                    store_routine(&mv!(entry).path(), config, user_choice)?;
                }
                // println!(
                //     "this path (dir - dir) is not fully developed mabye you have to delete something yourself"
                // );
                // println!("{:?}", target_path);
            } //this is already good I guess ~later~ I was wrong...
            (PathState::NoExist, PathState::Dir) => {
                copy_dir_all(&path_to_store, target_path)?; //dangerous
                delete_all(&path_to_store)?; //really dangerous
                store_routine(target_path, config, user_choice)?;
                //this is stupidly dangerous in case I wrote the copy wrong somewhere
            }
        }
        break;
    }
    Ok(())
}

fn classify_path(path: &PathBuf) -> PathState {
    if path.exists() {
        if path.is_symlink() {
            PathState::Symlink
        } else if path.is_dir() {
            PathState::Dir
        } else {
            PathState::File
        }
    } else {
        PathState::NoExist
    }
}

fn absolute_to_relative(absolute_path: &PathBuf) -> PathBuf {
    //strip_prefix could be better :/
    let mut temp = absolute_path.to_str().unwrap().to_string();
    temp.remove(0);
    PathBuf::from(temp)
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<(), String> {
    if !dst.exists() {
        mv!(fs::create_dir_all(dst));
    }
    for entry in mv!(fs::read_dir(src)) {
        let path = mv!(entry).path();
        let rel_path = path.strip_prefix(src).unwrap();
        let dest_path = dst.join(rel_path);

        if path.is_file() {
            mv!(fs::copy(path, dest_path));
        } else if path.is_dir() {
            copy_dir_all(&path, &dest_path)?;
        }
    }
    Ok(())
}

fn delete_all(path: &PathBuf) -> Result<(), String> {
    mv!(fs::remove_dir_all(path));
    Ok(())
}

pub enum UserChoice {
    TakeStore,
    TakeTarget,
    TakeStoreAll,
    TakeTargetAll,
    NoChoice,
}

fn get_user_choice(user_choice: &mut UserChoice) -> Result<(), String> {
    let mut input = String::new();
    println!("there are two files make choice");
    println!("1:take store(default)");
    println!("2:take target");
    println!("3:take store for all of this entry");
    println!("4:take target for all of this entry");

    mv!(stdin().read_line(&mut input));

    match input.trim() {
        "" | "1" => *user_choice = UserChoice::TakeStore,
        "2" => *user_choice = UserChoice::TakeTarget,
        "3" => *user_choice = UserChoice::TakeStoreAll,
        "4" => *user_choice = UserChoice::TakeTargetAll,
        _ => {
            return Err(format!("this is very wrong"));
        }
    }
    Ok(())
}
