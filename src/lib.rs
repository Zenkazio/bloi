use std::{
    fmt, fs,
    io::stdin,
    os::unix::fs::symlink,
    path::{PathBuf, StripPrefixError},
};

use thiserror::Error;

#[derive(Error)]
pub enum Error {
    #[error("Io: {0}")]
    Io(#[from] std::io::Error),
    #[error("SerdeJson: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("HomeDirNotFound: Home dir could not be found")]
    HomeDirNotFound,
    #[error("UnconventionalClapArgMissing: Parameter is missing: {0}")]
    UnconventionalClapArgMissing(String),
    #[error("PathNotClassified: Path could not be classified: {0}")]
    PathNotClassified(PathBuf),
    #[error("OtherStripPrefixError: {0}")]
    OtherStripPrefixError(#[from] StripPrefixError),
    #[error("EqNoExistDirError: this should not happen\n{0}")]
    EqNoExistDirError(PathBuf),
    #[error("EqSymLinkWithoutSource: Symlink without source potential data loss\n{0}")]
    EqSymLinkWithoutSource(PathBuf),
    #[error("EqSymLinkSymLink: Two Symlinks potential data loss\n{0}\n{1}")]
    EqSymLinkSymLink(PathBuf, PathBuf),
    #[error("EqNoExistNoExist: very strange no exist\n{0}\n{1}")]
    EqNoExistNoExist(PathBuf, PathBuf),
    #[error("EqFileSymLinkDir: PathTypeError\n{0}\n{1}")]
    EqFileSymLinkDir(PathBuf, PathBuf),
    #[error("NoPossibleUserChoice: Invalid selection: {0}")]
    NoPossibleUserChoice(String),
    #[error("NoParent: Directory has no parent")]
    NoParent,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // nutze Display-Text auch für Debug
        write!(f, "{}", self)
    }
}
pub type Result<T> = core::result::Result<T, Error>;

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
    println!("Storing: {:?}", target_path);
    let mut user_choice = UserChoice::NoChoice;
    let path_to_store = store_path.join(absolute_to_relative(target_path));
    work_on_entry(target_path, &path_to_store, &mut user_choice)?;
    Ok(())
}

fn work_on_entry(
    target_path: &PathBuf,
    path_to_store: &PathBuf,
    user_choice: &mut UserChoice,
) -> Result<()> {
    match eqalize(
        target_path,
        &path_to_store,
        &EqChoice::SymLinkInOne,
        user_choice,
    ) {
        Ok(_) => {}
        Err(e) => match e {
            Error::EqNoExistDirError(_) => {}
            e => return Err(e),
        },
    };
    eqalize(
        &path_to_store,
        target_path,
        &EqChoice::SymLinkInTwo,
        user_choice,
    )?;
    Ok(())
}

fn eqalize(
    path1: &PathBuf,
    path2: &PathBuf,
    eq_choice: &EqChoice,
    user_choice: &mut UserChoice,
) -> Result<()> {
    let path_type1 = classify_path(path1)?;
    let path_type2 = classify_path(path2)?;

    match (path_type1, path_type2) {
        (PathType::File, PathType::NoExist) => {
            make_dir_all_file(path2)?;
            match eq_choice {
                EqChoice::Copy => copy_file(path1, path2)?,
                EqChoice::SymLinkInOne => {
                    copy_file(path1, path2)?;
                    delete_file(path1)?;
                    create_symlink(path2, path1)?;
                }
                EqChoice::SymLinkInTwo => create_symlink(path1, path2)?,
            }
        }
        (PathType::NoExist, PathType::File) => {
            make_dir_all_file(path1)?;
            match eq_choice {
                EqChoice::Copy => copy_file(path2, path1)?,
                EqChoice::SymLinkInOne => create_symlink(path2, path1)?,
                EqChoice::SymLinkInTwo => {
                    copy_file(path2, path1)?;
                    delete_file(path2)?;
                    create_symlink(path1, path2)?;
                }
            }
        }
        (PathType::Dir, PathType::NoExist) => {
            make_dir_all(path2)?;
            let children = get_child_suffixes(path1)?;
            for child in children {
                eqalize(
                    &path1.join(&child),
                    &path2.join(&child),
                    eq_choice,
                    user_choice,
                )?;
            }
        }
        (PathType::NoExist, PathType::Dir) => {
            return Err(Error::EqNoExistDirError(path2.to_path_buf()));
        }
        (PathType::SymLink, PathType::NoExist) => {
            return Err(Error::EqSymLinkWithoutSource(path1.to_path_buf()));
        }
        (PathType::NoExist, PathType::SymLink) => {
            return Err(Error::EqSymLinkWithoutSource(path2.to_path_buf()));
        }
        (PathType::File, PathType::SymLink) => match eq_choice {
            EqChoice::Copy => {
                delete_file(path2)?;
                copy_file(path1, path2)?;
            }
            EqChoice::SymLinkInOne => {
                delete_file(path2)?;
                copy_file(path1, path2)?;
                delete_file(path1)?;
                create_symlink(path2, path1)?;
            }
            EqChoice::SymLinkInTwo => {
                delete_file(path2)?;
                create_symlink(path1, path2)?;
            }
        },
        (PathType::SymLink, PathType::File) => match eq_choice {
            EqChoice::Copy => {
                delete_file(path1)?;
                copy_file(path2, path1)?;
            }
            EqChoice::SymLinkInOne => {
                delete_file(path1)?;
                create_symlink(path2, path1)?;
            }
            EqChoice::SymLinkInTwo => {
                delete_file(path1)?;
                copy_file(path2, path1)?;
                delete_file(path2)?;
                create_symlink(path1, path2)?;
            }
        },
        (PathType::SymLink, PathType::SymLink) => {
            return Err(Error::EqSymLinkSymLink(
                path1.to_path_buf(),
                path2.to_path_buf(),
            ));
        }
        (PathType::NoExist, PathType::NoExist) => {
            return Err(Error::EqNoExistNoExist(
                path1.to_path_buf(),
                path2.to_path_buf(),
            ));
        }
        (PathType::File, PathType::Dir)
        | (PathType::Dir, PathType::File)
        | (PathType::SymLink, PathType::Dir)
        | (PathType::Dir, PathType::SymLink) => {
            return Err(Error::EqFileSymLinkDir(
                path1.to_path_buf(),
                path2.to_path_buf(),
            ));
        }
        (PathType::File, PathType::File) => {
            match *user_choice {
                UserChoice::NoChoice | UserChoice::TakeStore | UserChoice::TakeTarget => {
                    println!("{:?}", path1);
                    get_user_choice(user_choice)?;
                }
                _ => {} //skip it
            }
            match *user_choice {
                UserChoice::TakeStore | UserChoice::TakeStoreAll => {
                    delete_file(path1)?;
                    eqalize(path1, path2, eq_choice, user_choice)?;
                }
                UserChoice::TakeTarget | UserChoice::TakeTargetAll => {
                    delete_file(path2)?;
                    eqalize(path1, path2, eq_choice, user_choice)?;
                }
                _ => {}
            }
        }
        (PathType::Dir, PathType::Dir) => {
            let children = get_child_suffixes(path1)?;
            for child in children {
                eqalize(
                    &path1.join(&child),
                    &path2.join(&child),
                    eq_choice,
                    user_choice,
                )?;
            }
        }
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

fn create_symlink(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    Ok(symlink(src, dst)?)
}

fn copy_file(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::copy(src, dst)?;
    Ok(())
}
fn delete_file(path: &PathBuf) -> Result<()> {
    Ok(fs::remove_file(path)?)
}
// fn make_dir(path: &PathBuf) -> Result<()> {
//     fs::create_dir(path).map_err(Error::Io)
// }
fn make_dir_all(path: &PathBuf) -> Result<()> {
    Ok(fs::create_dir_all(path)?)
}
fn make_dir_all_file(path: &PathBuf) -> Result<()> {
    let parent = match path.parent() {
        Some(s) => s.to_path_buf(),
        None => return Err(Error::NoParent),
    };
    fs::create_dir_all(parent).map_err(Error::Io)
}
fn get_user_choice(user_choice: &mut UserChoice) -> Result<()> {
    let mut input = String::new();
    println!("File conflict detected! Both files exist in both locations.");
    println!("Please choose which version to keep:");
    println!("1: Use the version in storage (default)");
    println!("2: Use the version in the original location");
    println!("3: Use storage version for all remaining conflicts in this entry");
    println!("4: Use original version for all remaining conflicts in this entry");

    stdin().read_line(&mut input).map_err(Error::Io)?;

    match input.trim() {
        "" | "1" => *user_choice = UserChoice::TakeStore,
        "2" => *user_choice = UserChoice::TakeTarget,
        "3" => *user_choice = UserChoice::TakeStoreAll,
        "4" => *user_choice = UserChoice::TakeTargetAll,
        e => {
            return Err(Error::NoPossibleUserChoice(e.to_string()));
        }
    }
    Ok(())
}

fn get_child_suffixes(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let readdir = fs::read_dir(path).map_err(Error::Io)?;
    let mut vec = Vec::new();
    for entry in readdir {
        let entry = entry.map_err(Error::Io)?;
        vec.push(
            entry
                .path()
                .strip_prefix(path)
                .map_err(Error::OtherStripPrefixError)?
                .to_path_buf(),
        )
    }
    Ok(vec)
}

pub fn unstore_routine(target_path: &PathBuf, store_path: &PathBuf) -> Result<()> {
    println!("Unstoring: {:?}", target_path);
    let mut user_choice = UserChoice::NoChoice;
    let path_to_store = store_path.join(absolute_to_relative(target_path));
    eqalize(
        target_path,
        &path_to_store,
        &EqChoice::Copy,
        &mut user_choice,
    )?;
    eqalize(
        &path_to_store,
        target_path,
        &EqChoice::Copy,
        &mut user_choice,
    )?;
    delete_all(&path_to_store)?;
    Ok(())
}

fn delete_all(path: &PathBuf) -> Result<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}

#[cfg(test)]
#[allow(unused_must_use)]
fn test_setup() -> (PathBuf, PathBuf) {
    use std::io::Write;
    let (target_path, store_path) = test_teardown();

    make_dir_all(&target_path);
    make_dir_all(&target_path.join("folder1/"));
    make_dir_all(&target_path.join("folder1/").join("folder2/"));
    make_dir_all(&target_path);

    let mut file = fs::File::create(target_path.join("test_file1")).unwrap();
    file.write_all("FILE_CONTENT1".as_bytes());
    let mut file = fs::File::create(target_path.join("folder1/").join("test_file2")).unwrap();
    file.write_all("FILE_CONTENT2".as_bytes());
    let mut file = fs::File::create(
        target_path
            .join("folder1/")
            .join("folder2/")
            .join("test_file3"),
    )
    .unwrap();
    file.write_all("FILE_CONTENT3".as_bytes());
    (target_path, store_path)
}
#[cfg(test)]
#[allow(unused_must_use)]
fn test_teardown() -> (PathBuf, PathBuf) {
    let path = PathBuf::from("/home/zenkazio/Projects/bloi/TestEnv");
    let env1 = path.join("target/");
    let env2 = path.join("store/");
    delete_all(&env1);
    delete_all(&env2);
    (env1, env2)
}

#[cfg(test)]
#[test]
#[allow(unused_must_use)]
fn test_lib_store_routine() {
    let (target_path, store_path) = test_setup();
    store_routine(&target_path, &store_path).unwrap();
    test_teardown();
}

#[cfg(test)]
#[test]
#[allow(unused_must_use)]
fn test_lib_unstore_routine() {
    let (target_path, store_path) = test_setup();
    store_routine(&target_path, &store_path);
    unstore_routine(&target_path, &store_path);
    test_teardown();
}
