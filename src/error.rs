use std::path::{PathBuf, StripPrefixError};

#[derive(Debug)]
pub enum Error {
    Generic(String),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
    HomeDirNotFound,              // Could be "Unable to locate your home directory"
    GitPotentialConflict,         // Could be "Git conflict detected"
    UnconventionalClapArgMissing, // Could be "Required command argument missing"
    PathNotClassified(PathBuf),   // Could add "Path type cannot be determined"
    PathWasNotAbsolute(StripPrefixError), // "Path must be absolute"
    OtherStripPrefixError(StripPrefixError),
    EqNoExistDirError, // "Cannot create directory in non-existent location"
    EqSymLinkWithoutSource(PathBuf), // "Symlink points to non-existent source"
    EqSymLinkSymLink(PathBuf, PathBuf), // "Both paths are symlinks"
    EqNoExistNoExist(PathBuf, PathBuf), // "Neither source nor destination exists"
    EqFileSymLinkDir(PathBuf, PathBuf), // "Type mismatch between paths"
    NoPossibleUserChoice(String), // "Invalid selection"
}

pub type Result<T> = core::result::Result<T, Error>;
