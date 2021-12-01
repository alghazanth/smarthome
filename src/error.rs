use std::error::Error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, SmartHomeError>;

#[derive(Debug, PartialEq)]
pub enum SmartHomeError {
    NonExistentDevice,
    AlreadyExistentDevice,
    NonExistentRoom,
    AlreadyExistentRoom,
    BrokenSocket,
}

impl fmt::Display for SmartHomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Error for SmartHomeError {}
