//! # PGS Memory Buffer
//!
//! This module defines the `PgsMemoryBuffer`, which represents an in-memory buffer that can be
//! read from and seeked into. It also includes functionality for reading different byte orders.
use std::{fmt::Debug, io::Read};

use crate::{pgs_error::Result, PgsSeek};

/// A trait for handling different byte orders.
///
/// This trait defines methods for reading unsigned integers in various byte orders.
pub trait ByteOrder: Default + Debug + Clone {
    /// Reads a 16-bit unsigned integer from a byte slice.
    ///
    /// # Arguments
    /// * `buf` - A byte slice containing the data to read.
    ///
    /// # Returns
    /// Returns a `Result` containing the 16-bit integer on success, or an `Error` if the read operation fails.
    fn read_u16(buf: &[u8]) -> Result<u16>;

    /// Reads a 24-bit unsigned integer from a byte slice.
    ///
    /// # Arguments
    /// * `buf` - A byte slice containing the data to read.
    ///
    /// # Returns
    /// Returns a `Result` containing the 24-bit integer on success, or an `Error` if the read operation fails.
    fn read_u24(buf: &[u8]) -> Result<u32>;

    /// Reads a 32-bit unsigned integer from a byte slice.
    ///
    /// # Arguments
    /// * `buf` - A byte slice containing the data to read.
    ///
    /// # Returns
    /// Returns a `Result` containing the 32-bit integer on success, or an `Error` if the read operation fails.
    fn read_u32(buf: &[u8]) -> Result<u32>;
}

/// A struct representing the big-endian byte order.
#[derive(Clone, Copy, Debug)]
pub enum BigEndian {}

impl Default for BigEndian {
    fn default() -> BigEndian {
        panic!("BigEndian")
    }
}

impl ByteOrder for BigEndian {
    #[inline]
    fn read_u16(buf: &[u8]) -> Result<u16> {
        Ok(u16::from_be_bytes(buf[..2].try_into()?))
    }

    #[inline]
    fn read_u24(buf: &[u8]) -> Result<u32> {
        let mut out = [0; 4];
        out[1..].copy_from_slice(&buf[..3]);
        Ok(u32::from_be_bytes(out))
    }

    #[inline]
    fn read_u32(buf: &[u8]) -> Result<u32> {
        Ok(u32::from_be_bytes(buf[..4].try_into()?))
    }
}


/// A struct representing the little-endian byte order.
#[derive(Clone, Copy, Debug)]
pub enum LittleEndian {}

impl Default for LittleEndian {
    fn default() -> LittleEndian {
        panic!("LittleEndian")
    }
}

impl ByteOrder for LittleEndian {
    #[inline]
    fn read_u16(buf: &[u8]) -> Result<u16> {
        Ok(u16::from_le_bytes(buf[..2].try_into()?))
    }

    #[inline]
    fn read_u24(buf: &[u8]) -> Result<u32> {
        let mut out = [0; 4];
        out[..3].copy_from_slice(&buf[..3]);
        Ok(u32::from_le_bytes(out))
    }

    #[inline]
    fn read_u32(buf: &[u8]) -> Result<u32> {
        Ok(u32::from_le_bytes(buf[..4].try_into()?))
    }
}

/// A trait for reading bytes with support for different byte orders.
pub trait ReadBytes: Read {
    /// Reads a single 8-bit unsigned integer.
    ///
    /// # Returns
    /// Returns a `Result` containing the 8-bit integer on success, or an `Error` if the read operation fails.
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf: [u8; 1] = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Reads a 16-bit unsigned integer using the specified byte order.
    ///
    /// # Type Parameters
    /// * `T` - The byte order to use for reading the integer.
    ///
    /// # Returns
    /// Returns a `Result` containing the 16-bit integer on success, or an `Error` if the read operation fails.
    #[inline]
    fn read_u16<T: ByteOrder>(&mut self) -> Result<u16> {
        let mut buf: [u8; 2] = [0; 2];
        self.read_exact(&mut buf)?;
        T::read_u16(&buf)
    }

    /// Reads a 24-bit unsigned integer using the specified byte order.
    ///
    /// # Type Parameters
    /// * `T` - The byte order to use for reading the integer.
    ///
    /// # Returns
    /// Returns a `Result` containing the 24-bit integer on success, or an `Error` if the read operation fails.
    #[inline]
    fn read_u24<T: ByteOrder>(&mut self) -> Result<u32> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf)?;
        T::read_u24(&buf)
    }

    /// Reads a 32-bit unsigned integer using the specified byte order.
    ///
    /// # Type Parameters
    /// * `T` - The byte order to use for reading the integer.
    ///
    /// # Returns
    /// Returns a `Result` containing the 32-bit integer on success, or an `Error` if the read operation fails.
    #[inline]
    fn read_u32<T: ByteOrder>(&mut self) -> Result<u32> {
        let mut buffer: [u8; 4] = [0; 4];
        self.read_exact(&mut buffer)?;
        T::read_u32(&buffer)
    }
}

impl<R: Read + ?Sized> ReadBytes for R {}

/// A memory buffer that supports reading and seeking operations.
#[derive(Default)]
pub struct PgsMemoryBuffer {
    buffer: Vec<u8>,
    position: usize,
}

impl PgsMemoryBuffer {
    /// Creates a new `PgsMemoryBuffer` instance.
    ///
    /// # Returns
    /// Returns a new `PgsMemoryBuffer` with an empty buffer and position set to 0.
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
        }
    }

    /// Reads a fixed number of bytes into a fixed-size array.
    ///
    /// # Arguments
    /// * `N` - The number of bytes to read, specified as a constant generic parameter.
    ///
    /// # Returns
    /// Returns a `Result` containing an array of bytes on success, or an `Error` if the read operation fails.
    pub fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut buffer: [u8; N] = [0; N];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }
    
    /// Reads a specified number of bytes into a `Vec<u8>`.
    ///
    /// # Arguments
    /// * `length` - The number of bytes to read.
    ///
    /// # Returns
    /// Returns a `Result` containing a vector of bytes on success, or an `Error` if the read operation fails.
    pub fn read_into_vec(&mut self, length: u32) -> Result<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![0; length as usize];
        let buffer_slice = buffer.as_mut_slice();
        self.read_exact(buffer_slice)?;
        Ok(buffer)
    }

    /// Returns a slice of the remaining bytes in the buffer.
    ///
    /// # Returns
    /// Returns a slice of the remaining bytes starting from the current position.
    pub fn remaining_slice(&self) -> &[u8] {
        let start_pos = self.position.min(self.buffer.len());
        &self.buffer.as_slice()[(start_pos)..]
    }
}

impl PgsSeek for PgsMemoryBuffer {
    fn seek(&mut self, to: usize) -> Result<usize> {
        self.position = to;
        Ok(self.position)
    }

    fn pos(&mut self) -> Result<usize> {
        Ok(self.position)
    }

    fn len(&self) -> Result<usize> {
        Ok(self.buffer.len())
    }
}

impl Read for PgsMemoryBuffer {
    /// Reads bytes from the buffer into the provided slice.
    ///
    /// # Arguments
    /// * `buffer` - A mutable byte slice to read into.
    ///
    /// # Returns
    /// Returns a `Result` containing the number of bytes read on success, or an `Error` if the read operation fails.
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let len = Read::read(&mut self.remaining_slice(), buffer)?;
        self.position += len;
        Ok(len)
    }

    /// Reads bytes from the buffer into the provided slice, ensuring that the exact number of bytes is read.
    ///
    /// # Arguments
    /// * `buffer` - A mutable byte slice to read into.
    ///
    /// # Returns
    /// Returns a `Result` indicating success or failure of the read operation.
    fn read_exact(&mut self, buffer: &mut [u8]) -> std::io::Result<()> {
        let buf_len = buffer.len();
        Read::read_exact(&mut self.remaining_slice(), buffer)?;
        self.position += buf_len;
        Ok(())
    }
}

impl From<Vec<u8>> for PgsMemoryBuffer {
    /// Creates a `PgsMemoryBuffer` from a `Vec<u8>`.
    ///
    /// # Arguments
    /// * `buffer` - A `Vec<u8>` to initialize the `PgsMemoryBuffer`.
    ///
    /// # Returns
    /// Returns a `PgsMemoryBuffer` containing the provided vector.
    fn from(buffer: Vec<u8>) -> Self {
        PgsMemoryBuffer {
            buffer,
            position: 0,
        }
    }
}

impl From<&[u8]> for PgsMemoryBuffer {
    /// Creates a `PgsMemoryBuffer` from a byte slice.
    ///
    /// # Arguments
    /// * `buffer` - A byte slice to initialize the `PgsMemoryBuffer`.
    ///
    /// # Returns
    /// Returns a `PgsMemoryBuffer` containing the provided slice.    
    fn from(buffer: &[u8]) -> Self {
        PgsMemoryBuffer {
            buffer: Vec::from(buffer),
            position: 0,
        }
    }
}