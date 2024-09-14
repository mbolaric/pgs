//! # Error Handling Module
//!
//! This module defines the error types and result handling mechanisms used throughout the library.
//! It includes custom errors specific to PGS processing and reuses standard Rust I/O errors where appropriate.
use core::fmt;
use std::array::TryFromSliceError;

/// Enum representing different error types used in the library.
///
/// Variants:
/// - `File(std::io::Error)`: Represents an error encountered while performing I/O operations.
/// - `InvalidInputArray`: Indicates that an input array is invalid.
/// - `ReadInvalidSegment`: Read operation encountered an invalid segment.
/// - `InvalidSegmentDataLength`: Segment has an incorrect data length.
/// - `IncompleteDisplaySet`: Indicates that the display set is incomplete.
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

/// A custom result type used throughout the library.
pub type Result<T> = core::result::Result<T, Error>;