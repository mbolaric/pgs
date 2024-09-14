use std::{fs::{File, Metadata}, io::{Read, Seek}};

use crate::pgs_error::{Result, Error};

/// A wrapper around a `File` that includes file metadata and provides methods to read data.
///
/// This struct provides methods to read bytes from a file and check if the end of the file has been reached.
/// It also maintains file metadata for boundary checking.
#[derive(Debug)]
pub struct PgsFile {
    file: File,
    metadata: Metadata
}

impl PgsFile {
    /// Creates a new `PgsFile` instance.
    ///
    /// # Arguments
    /// * `file` - The `File` instance to be wrapped by `PgsFile`.
    ///
    /// # Returns
    /// Returns a `Result` containing either a `PgsFile` instance or an `Error` if file metadata retrieval fails.
    pub(crate) fn new(file: File) -> Result<Self> {
        Ok(PgsFile {
            metadata: file.metadata()?,
            file
        })
    }

    /// Returns a reference to the file metadata.
    ///
    /// # Returns
    /// Returns a reference to the `Metadata` associated with the file.    
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Reads bytes from the file into the provided buffer.
    ///
    /// # Arguments
    /// * `buffer` - A mutable byte slice where the data will be read into.
    ///
    /// # Returns
    /// Returns a `Result` indicating success or an `Error` if reading the bytes fails or if the read operation would
    /// exceed the file's length.    
    pub fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<()> {
        if self.file.stream_position()? + buffer.len() as u64 > self.metadata.len() {
            return Err(Error::File(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "end of file")));
        }
        self.file.read_exact(buffer)?;
        Ok(())
    }

    /// Reads a fixed number of bytes from the file into a fixed-size array.
    ///
    /// # Type Parameters
    /// * `N` - The number of bytes to read, defined as a constant generic parameter.
    ///
    /// # Returns
    /// Returns a `Result` containing either a fixed-size array of bytes or an `Error` if reading the bytes fails or if
    /// the read operation would exceed the file's length.
    pub fn read_n_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        if self.file.stream_position()? + N as u64 > self.metadata.len() {
            return Err(Error::File(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "end of file")));
        }
        let mut buffer: [u8; N] = [0; N];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Checks if the current position in the file is at or past the end of the file.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a boolean value. `true` indicates that the end of the file has been reached or
    /// exceeded, while `false` indicates that there is more data to read.
    pub fn is_eof(&mut self) -> Result<bool> {
        Ok(self.file.stream_position()? >= self.metadata.len())
    }
}
