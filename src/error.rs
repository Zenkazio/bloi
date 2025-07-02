use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
    HomeDirNotFound,
    GitPotentialConflict,
}

pub type Result<T> = core::result::Result<T, Error>;
