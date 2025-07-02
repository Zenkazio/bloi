use std::path::{PathBuf, StripPrefixError};

#[derive(Debug)]
pub enum Error {
    Generic(String),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
    HomeDirNotFound,
    GitPotentialConflict,
    UnconventionalClapArgMissing,
    PathNotClassified(PathBuf),
    PathWasNotAbsolute(StripPrefixError),
}

pub type Result<T> = core::result::Result<T, Error>;
