use std::convert::From;
use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    GitError(git2::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Self {
        Error::GitError(error)
    }
}
