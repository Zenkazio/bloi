use std::{fs, io, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
enum PathType {
    Dir,
    File,
    SymLink,
    NoExist,
}
#[derive(Debug)]
struct PathTree {
    root: PathBuf,
    children: Vec<PathTree>,
    path_type: PathType,
}

impl PathTree {
    fn new(path: PathBuf) -> Result<Self, io::Error> {
        let path_type = classify_path(&path);
        let mut children = Vec::new();

        match path_type {
            PathType::Dir => {
                for entry in fs::read_dir(&path)? {
                    children.push(PathTree::new(entry?.path())?);
                }
                return Ok(Self {
                    root: path,
                    children: children,
                    path_type: path_type,
                });
            }
            _ => {
                return Ok(Self {
                    root: path,
                    children: children,
                    path_type: path_type,
                });
            }
        }
    }
}

fn classify_path(path: &PathBuf) -> PathType {
    if path.exists() {
        if path.is_symlink() {
            PathType::SymLink
        } else if path.is_dir() {
            PathType::Dir
        } else {
            PathType::File
        }
    } else {
        PathType::NoExist
    }
}

#[cfg(test)]
mod tests {
    use std::{io, path::PathBuf};

    use crate::path_tree::{self, PathTree, PathType};

    #[test]
    fn create_path_tree() -> Result<(), io::Error> {
        let path_file = PathBuf::from("/home/zenkazio/Projects/bloi/src/path_tree.rs");
        let path_dir = PathBuf::from("/home/zenkazio/Projects/bloi/");
        let path_symlink = PathBuf::from("/home/zenkazio/.config/niri/config.kdl");
        let path_noexist = PathBuf::from("/home/zenkazio/Projects/bloi/src/schlobo");

        let path_tree_file = PathTree::new(path_file)?;
        let path_tree_dir = PathTree::new(path_dir)?;
        let path_tree_symlink = PathTree::new(path_symlink)?;
        let path_tree_noexist = PathTree::new(path_noexist)?;

        assert_eq!(PathType::File, path_tree_file.path_type);
        assert_eq!(PathType::Dir, path_tree_dir.path_type);
        assert_eq!(PathType::SymLink, path_tree_symlink.path_type);
        assert_eq!(PathType::NoExist, path_tree_noexist.path_type);

        Ok(())
    }
}
