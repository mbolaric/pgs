use core::fmt;
use std::array::TryFromSliceError;

#[derive(Debug)]
pub enum Error {
    File(std::io::Error),
    InvalidInputArray,
    ReadInvalidSegment,
    InvalidSegmentDataLength,
    IncompleteDisplaySet
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::File(value)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(_: TryFromSliceError) -> Self {
        Error::InvalidInputArray
    }
}

pub type Result<T> = core::result::Result<T, Error>;