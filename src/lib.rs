use crate::error::*;
use std::path::{self, PathBuf};

mod error;

#[derive(Debug)]
enum PathType {
    SymLink,
    NoExist,
    File,
    Dir,
}

enum UserChoice {
    TakeStore,
    TakeTarget,
    TakeStoreAll,
    TakeTargetAll,
    NoChoice,
}

enum EqChoice {
    SymLinkInOne,
    SymLinkInTwo,
    Copy,
}

pub fn store_routine(target_path: &PathBuf, store_path: &PathBuf) -> Result<()> {
    let mut user_choice = UserChoice::NoChoice;
    work_on_entry(target_path, store_path, &mut user_choice);
    Ok(())
}

fn work_on_entry(target_path: &PathBuf, store_path: &PathBuf, user_choice: &mut UserChoice) {
    let path_to_store = store_path.join(absolute_to_relative(target_path));
    eqalize(
        target_path,
        &path_to_store,
        &EqChoice::SymLinkInOne,
        user_choice,
    );
    eqalize(
        &path_to_store,
        target_path,
        &EqChoice::SymLinkInTwo,
        user_choice,
    );
}

fn eqalize(
    path1: &PathBuf,
    path2: &PathBuf,
    eq_choice: &EqChoice,
    user_choice: &mut UserChoice,
) -> Result<()> {
    let path_type1 = classify_path(path1)?;
    let path_type2 = classify_path(path2)?;
    loop {
        match (path_type1, path_type2) {
            (PathType::File, PathType::NoExist) => {}
        }
        break;
    }
    Ok(())
}

fn classify_path(path: &PathBuf) -> Result<PathType> {
    if path.exists() {
        if path.is_symlink() {
            Ok(PathType::SymLink)
        } else if path.is_dir() {
            Ok(PathType::Dir)
        } else if path.is_file() {
            Ok(PathType::File)
        } else {
            Err(Error::PathNotClassified(path.clone()))
        }
    } else {
        Ok(PathType::NoExist)
    }
}

fn absolute_to_relative(absolute_path: &PathBuf) -> PathBuf {
    if !absolute_path.is_absolute() {
        return absolute_path.to_path_buf();
    }
    //Error has been handeled
    absolute_path.strip_prefix("/").unwrap().to_path_buf()
}
