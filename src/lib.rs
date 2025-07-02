use std::path::PathBuf;

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
fn classify_path(path: &PathBuf) -> Result<PathType, String> {
    if path.exists() {
        if path.is_symlink() {
            Ok(PathType::SymLink)
        } else if path.is_dir() {
            Ok(PathType::Dir)
        } else {
            Ok(PathType::File)
        }
    } else {
        Ok(PathType::NoExist)
    }
}
