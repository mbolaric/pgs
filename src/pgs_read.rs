//! # PGS Read Trait
//!
//! Defines traits for reading data from PGS files or buffers, with support for seeking.
use std::io::Read;

use crate::pgs_error::Result;

/// A trait for seeking within a read/write context.
///
/// This trait provides methods for seeking to a specific position, querying the current position, getting the length
/// of the data, and checking if the data is non-empty.
pub trait PgsSeek {
    /// Seeks to a specific position in the data.
    ///
    /// # Arguments
    /// * `to` - The position to seek to, specified as an offset from the beginning.
    ///
    /// # Returns
    /// Returns a `Result` containing the new position on success, or an `Error` if the seek operation fails.
    fn seek(&mut self, to: usize) -> Result<usize>;
    
    /// Gets the current position in the data.
    ///
    /// # Returns
    /// Returns a `Result` containing the current position on success, or an `Error` if the position retrieval fails.
    fn pos(&mut self) -> Result<usize>;

    /// Gets the total length of the data.
    ///
    /// # Returns
    /// Returns a `Result` containing the length of the data on success, or an `Error` if length retrieval fails.
    fn len(&self) -> Result<usize>;

    /// Checks if the data is non-empty.
    ///
    /// This is a convenience method that checks if the length of the data is greater than 0.
    ///
    /// # Returns
    /// Returns `true` if the length of the data is greater than 0, otherwise `false`.
    fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) > 0
    }
}

/// A trait for reading data with seeking capabilities.
///
/// This trait combines `Read` from the standard library and `PgsSeek`, indicating that a type implementing this trait
/// can both read data and seek within it.
pub trait PgsRead: Read + PgsSeek {}