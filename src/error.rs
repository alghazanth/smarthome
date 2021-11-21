use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    NonExistentDevice,
    AlreadyExistentDevice,
    NonExistentRoom,
    AlreadyExistentRoom,
    BrokenSocket,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}
