use crate::{error::Error, prelude::*};
use std::{fs, os::unix::fs::symlink, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub enum PathState {
    NotExist,
    Dir,
    File,
    Symlink,
}
#[derive(Debug, PartialEq, Eq)]
pub enum State {
    ///file/dir is symlink and store contains file/dir nothing needs to be done
    Stored,
    ///file/dir exists (not as symlink) and does not exist in store and needs to be stored
    ToBeStored,
    ///file/dir does not exist but exists in store and just symlinks need to be create (dirs too)
    Restock,
    ///file/dir exists and does exists in store user needs to choose one of the first to states
    Confilct,
    //file/dir are already symlink but nothing is in store this is a hard error
    //_Error,
}

pub fn decide_state_and_proccess_path(
    target_path: &PathBuf,
    store_path: &PathBuf,
    state: Option<State>,
) -> Result<Option<State>> {
    let path_to_store = create_path_to_store(target_path, store_path)?;
    let target_state: PathState;
    let to_store_state: PathState;

    if target_path.exists() {
        if target_path.is_dir() {
            target_state = PathState::Dir;
        } else if target_path.is_symlink() {
            target_state = PathState::Symlink;
        } else if target_path.is_file() {
            target_state = PathState::File;
        } else {
            return Err(Error::Generic(format!(
                "target is nor dir nor file nor symlink"
            )));
        }
    } else {
        target_state = PathState::NotExist;
    }

    if path_to_store.exists() {
        if path_to_store.is_dir() {
            to_store_state = PathState::Dir;
        } else if path_to_store.is_file() {
            to_store_state = PathState::File;
        } else {
            return Err(Error::Generic(format!("store item is nor dir nor file")));
        }
    } else {
        to_store_state = PathState::NotExist;
    }
    match target_state {
        PathState::File => match to_store_state {
            PathState::File => Some(State::Confilct),
            PathState::Symlink => {
                return Err(Error::Generic(format!(
                    "stored item cannot be symlink\n{:?}",
                    path_to_store
                )));
            }
            PathState::Dir => {
                return Err(Error::Generic(format!(
                    "if target is file store cannot be dir\n{:?}",
                    path_to_store
                )));
            }
            PathState::NotExist => Some(State::ToBeStored),
        },
        PathState::Symlink => match to_store_state {
            PathState::File => Some(State::Stored),
            PathState::Symlink => {
                return Err(Error::Generic(format!(
                    "stored item cannot be symlink\n{:?}",
                    path_to_store
                )));
            }
            PathState::Dir => {
                return Err(Error::Generic(format!(
                    "if target is symlink store cannot be dir\n{:?}",
                    path_to_store
                )));
            }
            PathState::NotExist => {
                return Err(Error::Generic(format!(
                    "hard problem target is symlink but nothing is stored\n{:?}\n{:?}",
                    target_path, path_to_store
                )));
            }
        },
        PathState::NotExist => match to_store_state {
            PathState::File => Some(State::Restock),
            PathState::Symlink => {
                return Err(Error::Generic(format!(
                    "stored item cannot be symlink\n{:?}",
                    path_to_store
                )));
            }
            PathState::Dir => Some(State::Restock),
            PathState::NotExist => {
                return Err(Error::Generic(format!(
                    "nothing exists config.json needs to be added"
                )));
            }
        },
        PathState::Dir => match to_store_state {
            PathState::File => {
                return Err(Error::Generic(format!("impossible case")));
            }
            PathState::Symlink => {
                return Err(Error::Generic(format!("impossible case")));
            }
            PathState::Dir => Some(State::Stored),
            PathState::NotExist => Some(State::ToBeStored),
        },
    };
    Ok(None)
}

pub fn process_path_based_on_type(target_path: &PathBuf, path_to_store: &PathBuf) -> Result<()> {
    if !target_path.exists() {
        return Err(Error::Generic(format!(
            "Path does not exist: {}",
            target_path.display()
        )));
    }
    if target_path.is_dir() {
        if let Ok(temp) = fs::read_dir(&target_path) {
            for entry in temp {
                if let Ok(entry) = entry {
                    process_path_based_on_type(&entry.path(), path_to_store);
                }
            }
        }
    } else if target_path.is_file() || target_path.is_symlink() {
        dbg!(&target_path);
        move_file_and_create_symlink(&target_path, &add_dotfiles_segment(&target_path)?);
    } else {
        return Err(Error::Generic(format!("what?: {}", target_path.display())));
    }
    Ok(())
}
pub fn move_file_and_create_symlink(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    dbg!(&src, &dst);
    if dst.is_file() {
        if src.is_symlink() {
            if let Ok(link_target) = fs::read_link(&src) {
                if &link_target == dst {
                    return Ok(());
                }
            }
        }
    }
    if !src.exists() {
        return Err(Error::Generic(format!(
            "Source path does not exist: {}",
            src.display()
        )));
    }
    if !src.is_file() {
        return Err(Error::Generic(format!(
            "Source path is not a file: {}",
            src.display()
        )));
    }
    let original_path_for_symlink = src.to_path_buf();

    if let Some(parent) = dst.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent);
        }
    }

    fs::rename(src, &dst);
    symlink(dst, &original_path_for_symlink);
    Ok(())
}

pub fn add_dotfiles_segment(original_path: &PathBuf) -> Result<PathBuf> {
    let mut components = original_path.components();
    let mut new_path_buf = PathBuf::new();

    if let Some(comp) = components.next() {
        new_path_buf.push(comp);
    } else {
        return Err(Error::Generic("irgendwas mit comps?!".to_string()));
    }
    if let Some(comp) = components.next() {
        new_path_buf.push(comp);
    } else {
        return Err(Error::Generic("irgendwas mit comps?!".to_string()));
    }

    if let Some(comp) = components.next() {
        new_path_buf.push(comp);
        new_path_buf.push(".dotfiles");
    } else {
        return Err(Error::Generic("irgendwas mit comps?!".to_string()));
    }

    for component in components {
        new_path_buf.push(component);
    }

    Ok(new_path_buf)
}

pub fn create_dir(dir: &PathBuf) -> Result<()> {
    match fs::create_dir_all(dir) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(Error::Generic(format!("Error creating dir(s): {:?}", e))),
    }
}

pub fn create_path_to_store(target_path: &PathBuf, store_path: &PathBuf) -> Result<PathBuf> {
    let mut target_str = target_path.to_str().unwrap().to_string();
    target_str.remove(0);
    Ok(store_path.clone().join(PathBuf::from(target_str)))
}
