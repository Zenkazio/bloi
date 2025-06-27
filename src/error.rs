#[derive(Debug)]
pub enum Error {
    Generic(String),
    IO(std::io::Error),
}
