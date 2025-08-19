use std::{
    collections::HashSet,
    fmt,
    fs::{self, create_dir_all, remove_dir, remove_dir_all, remove_file},
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
    #[error("ConfigDirNotFound: Config dir could not be found")]
    ConfigDirNotFound,
    #[error(
        "GitPotentialConflict: Found a potential git conflict - merge must be performed by user"
    )]
    GitPotentialConflict,
    #[error("UnconventionalClapArgMissing: Parameter is missing: {0}")]
    UnconventionalClapArgMissing(String),
    #[error("PathNotClassified: Path could not be classified: {0}")]
    PathNotClassified(PathBuf),
    #[error("OtherStripPrefixError: {0}")]
    OtherStripPrefixError(#[from] StripPrefixError),
    #[error("EqNoExistDirError: this should not happen\n{0}\n{1}")]
    EqNoExistDirError(PathBuf, PathBuf),
    #[error("EqSymLinkWithoutSource: Symlink without source potential data loss\n{0}")]
    EqSymLinkWithoutSource(PathBuf),
    #[error("EqSymLinkSymLink: Two Symlinks potential data loss\n{0}\n{1}")]
    EqSymLinkSymLink(PathBuf, PathBuf),
    #[error("EqNoExistNoExist: very strange no exist\n{0}\n{1}")]
    EqNoExistNoExist(PathBuf, PathBuf),
    #[error("EqFileDir: PathTypeError\n{0}\n{1}")]
    EqFileDir(PathBuf, PathBuf),
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
    SymLink(PathBuf, PathBuf),
    NoExist(PathBuf),
    File(PathBuf),
    Dir(PathBuf),
}

enum UserChoice {
    TakeStore,
    TakeTarget,
    TakeStoreAll,
    TakeTargetAll,
    NoChoice,
}

pub fn store_routine(target_path: &PathBuf, store_path: &PathBuf) -> Result<()> {
    println!("Storing: {:?}", target_path);
    let path_to_store = &store_path.join(absolute_to_relative(target_path));

    if !check_link(&target_path, &path_to_store)? {
        println!("Must equalize!");
        match eqalize(target_path, path_to_store, &mut UserChoice::NoChoice) {
            Ok(_) => {}
            Err(e) => match e {
                Error::EqSymLinkWithoutSource(_)
                | Error::EqNoExistDirError(_, _)
                | Error::EqSymLinkSymLink(_, _)
                | Error::EqFileDir(_, _) => {
                    println!("{e}")
                }
                e => return Err(e),
            },
        };
        remove_special(target_path)?;
        symlink(path_to_store, target_path)?;
    }
    Ok(())
}

fn eqalize(path1: &PathBuf, path2: &PathBuf, user_choice: &mut UserChoice) -> Result<()> {
    let path_type1 = classify_path(path1)?;
    let path_type2 = classify_path(path2)?;

    match (path_type1, path_type2) {
        (PathType::File(p1), PathType::NoExist(p2))
        | (PathType::NoExist(p2), PathType::File(p1)) => {
            make_dir_all_file(&p2)?;
            copy(&p1, &p2)?
        }
        (PathType::NoExist(_), PathType::SymLink(p, _))
        | (PathType::SymLink(p, _), PathType::NoExist(_)) => {
            return Err(Error::EqSymLinkWithoutSource(p.to_path_buf()));
        }
        (PathType::File(p1), PathType::SymLink(p2, _))
        | (PathType::SymLink(p2, _), PathType::File(p1)) => {
            remove_file(&p2)?;
            copy(&p1, &p2)?;
        }
        (PathType::SymLink(p1, _), PathType::SymLink(p2, _)) => {
            return Err(Error::EqSymLinkSymLink(p1, p2));
        }
        (PathType::NoExist(p1), PathType::NoExist(p2)) => {
            return Err(Error::EqNoExistNoExist(p1, p2));
        }
        (PathType::File(p1), PathType::Dir(p2)) | (PathType::Dir(p1), PathType::File(p2)) => {
            return Err(Error::EqFileDir(p1, p2));
        }
        (PathType::SymLink(p1, _), PathType::Dir(_))
        | (PathType::Dir(_), PathType::SymLink(p1, _)) => {
            remove_file(&p1)?;
            eqalize(path1, path2, user_choice)?;
        }
        (PathType::File(p1), PathType::File(p2)) => {
            match *user_choice {
                UserChoice::NoChoice | UserChoice::TakeStore | UserChoice::TakeTarget => {
                    println!("{:?}", p1);
                    get_user_choice(user_choice)?;
                }
                _ => {} //skip it
            }
            match *user_choice {
                UserChoice::TakeStore | UserChoice::TakeStoreAll => {
                    remove_file(&p1)?;
                    eqalize(path1, path2, user_choice)?;
                }
                UserChoice::TakeTarget | UserChoice::TakeTargetAll => {
                    remove_file(&p2)?;
                    eqalize(path1, path2, user_choice)?;
                }
                _ => {}
            }
        }
        (PathType::Dir(p1), PathType::Dir(p2))
        | (PathType::NoExist(p1), PathType::Dir(p2))
        | (PathType::Dir(p1), PathType::NoExist(p2)) => {
            create_dir_all(&p1)?;
            create_dir_all(&p2)?;
            let children1 = get_child_suffixes(&p1)?;
            let children2 = get_child_suffixes(&p2)?;
            let children: HashSet<_> = children1.union(&children2).collect();
            for child in children {
                eqalize(&p1.join(child), &p2.join(child), user_choice)?;
            }
        }
    }

    Ok(())
}

fn classify_path(path: &PathBuf) -> Result<PathType> {
    if path.is_symlink() {
        let link_target = fs::read_link(&path)?;
        Ok(PathType::SymLink(path.to_path_buf(), link_target))
    } else {
        if path.exists() {
            if path.is_dir() {
                Ok(PathType::Dir(path.to_path_buf()))
            } else if path.is_file() {
                Ok(PathType::File(path.to_path_buf()))
            } else {
                Err(Error::PathNotClassified(path.to_path_buf()))
            }
        } else {
            Ok(PathType::NoExist(path.to_path_buf()))
        }
    }
}
fn check_link(link: &PathBuf, original: &PathBuf) -> Result<bool> {
    let link_type = classify_path(link)?;

    match link_type {
        PathType::SymLink(_, link1) => {
            if &link1 != original {
                Ok(false)
            } else {
                Ok(true)
            }
        }
        _ => Ok(false),
    }
}
pub fn absolute_to_relative(absolute_path: &PathBuf) -> PathBuf {
    if !absolute_path.is_absolute() {
        return absolute_path.to_path_buf();
    }
    //Error has been handeled
    absolute_path.strip_prefix("/").unwrap().to_path_buf()
}
fn remove_special(path: &PathBuf) -> Result<()> {
    let path_type = classify_path(path)?;
    match path_type {
        PathType::File(_) | PathType::SymLink(_, _) => Ok(remove_file(path)?),
        PathType::Dir(_) => Ok(remove_dir_all(path)?),
        PathType::NoExist(_) => Ok(remove_dir_all(path)?),
    }
}
///a wrapper for fs::copy changes u64 to ()
fn copy(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::copy(src, dst)?;
    Ok(())
}
pub fn make_dir_all_file(path: &PathBuf) -> Result<()> {
    let parent = match path.parent() {
        Some(s) => s.to_path_buf(),
        None => return Err(Error::NoParent),
    };
    create_dir_all(parent)?;
    Ok(())
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

fn get_child_suffixes(path: &PathBuf) -> Result<HashSet<PathBuf>> {
    let readdir = fs::read_dir(path).map_err(Error::Io)?;
    let mut vec = HashSet::new();
    for entry in readdir {
        let entry = entry.map_err(Error::Io)?;
        vec.insert(
            entry
                .path()
                .strip_prefix(path)
                .map_err(Error::OtherStripPrefixError)?
                .to_path_buf(),
        );
    }
    Ok(vec)
}

pub fn unstore_routine(target_path: &PathBuf, store_path: &PathBuf) -> Result<()> {
    println!("Unstoring: {:?}", target_path);
    let mut user_choice = UserChoice::NoChoice;
    let path_to_store = store_path.join(absolute_to_relative(target_path));

    eqalize(&path_to_store, target_path, &mut user_choice)?;
    remove_dir_all(&path_to_store)?;
    Ok(())
}

#[cfg(test)]
#[allow(unused_must_use)]
fn test_setup() -> (PathBuf, PathBuf) {
    use std::io::Write;
    let (target_path, store_path) = test_teardown();

    create_dir_all(&target_path);
    create_dir_all(&store_path);
    create_dir_all(&target_path.join("folder1/"));
    create_dir_all(&target_path.join("folder1/").join("folder2/"));
    create_dir_all(&target_path);

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
    remove_dir_all(&env1);
    remove_dir_all(&env2);
    (env1, env2)
}

// #[cfg(test)]
// #[test]
// #[allow(unused_must_use)]
// fn test_lib_store_routine() {
//     let (target_path, store_path) = test_setup();
//     store_routine(&target_path, &store_path).unwrap();
//     test_teardown();
// }

// #[cfg(test)]
// #[test]
// #[allow(unused_must_use)]
// fn test_lib_unstore_routine() {
//     let (target_path, store_path) = test_setup();
//     store_routine(&target_path, &store_path);
//     unstore_routine(&target_path, &store_path);
//     test_teardown();
// }

#[cfg(test)]
#[test]
#[allow(unused_must_use)]
fn test_classify_path() {
    let (target_path, _) = test_setup();
    let p1 = target_path.join("test_file1");
    let s1 = target_path.join("sym_test_file1");
    dbg!(classify_path(&target_path.clone()));
    dbg!(classify_path(&target_path.join("test_file1")));
    symlink(
        target_path.join("test_file1"),
        target_path.join("sym_test_file1"),
    );
    dbg!(classify_path(&target_path.join("sym_test_file1")));
    dbg!(classify_path(&target_path.join("test_file4")));
    // test_teardown();
    //
    remove_file(target_path.join("test_file1"));
    println!("____________________________");
    dbg!(classify_path(&p1));
    dbg!(classify_path(&s1.clone()));
    dbg!(fs::read_link(s1));
    test_teardown();
}
