use crate::{
    config::{get_dotfiles_path, get_home_path},
    error::Error,
    prelude::*,
};
use std::{
    fs,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

pub fn check_store_dir(store_dir: &PathBuf) -> Result<()> {
    fs::create_dir_all(store_dir).expect("could not create store dir");
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

pub fn process_path_based_on_type(p: &PathBuf, store_dir: &PathBuf) -> Result<()> {
    if !p.exists() {
        return Err(Error::Generic(format!(
            "Path does not exist: {}",
            p.display()
        )));
    }

    if p.is_dir() {
        if let Ok(temp) = fs::read_dir(&p) {
            for entry in temp {
                if let Ok(entry) = entry {
                    process_path_based_on_type(&entry.path(), store_dir);
                }
            }
        }
    } else if p.is_file() || p.is_symlink() {
        dbg!(&p);
        move_file_and_create_symlink(&p, &add_dotfiles_segment(&p)?);
    } else {
        return Err(Error::Generic(format!("what?: {}", p.display())));
    }
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
