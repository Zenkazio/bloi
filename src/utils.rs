use std::{fs, os::unix::fs::symlink, path::PathBuf};

use crate::config::Config;

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

pub fn store_routine(target_path: &PathBuf, config: &Config) -> Result<(), String> {
    let path_to_store: PathBuf = config.store_path.join(absolute_to_relative(target_path));
    let target_state = classify_path(target_path);
    let store_state = classify_path(&path_to_store);

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
            todo!()
        } //conflict which user needs to resolve
        (PathState::File, PathState::NoExist) => {
            mv!(fs::create_dir_all(path_to_store.parent().unwrap()));
            mv!(fs::rename(target_path, &path_to_store));
            mv!(symlink(path_to_store, target_path));
        } // this is the case for storing
        (PathState::NoExist, PathState::NoExist) => {
            return Err(format!(
                "nothing in target and store mabye old path in config\n{:?}",
                target_path
            ));
        }
        (PathState::Dir, PathState::NoExist) => {
            mv!(fs::create_dir_all(path_to_store));
            for entry in mv!(fs::read_dir(target_path)) {
                store_routine(&mv!(entry).path(), config)?;
            }
        }
        (PathState::NoExist, PathState::File) => {
            mv!(fs::create_dir_all(target_path.parent().unwrap()));
            mv!(symlink(path_to_store, target_path));
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
        (PathState::Dir, PathState::Dir) => {} //this is already good I guess
        (PathState::NoExist, PathState::Dir) => {
            mv!(fs::create_dir_all(target_path));
            todo!() // something store dir not on target need to travers store dir
        }
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
    let mut temp = absolute_path.to_str().unwrap().to_string();
    temp.remove(0);
    PathBuf::from(temp)
}
