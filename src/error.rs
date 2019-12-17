use std::convert::From;
use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    GitError(git2::Error),
    StripPrefixError,
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

impl From<std::path::StripPrefixError> for Error {
    fn from(_: std::path::StripPrefixError) -> Self {
        Error::StripPrefixError
    }
}
