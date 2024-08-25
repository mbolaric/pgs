use std::{fs::{File, Metadata}, io::{Read, Seek}};

use crate::pgs_error::{Result, Error};

#[derive(Debug)]
pub struct PgsFile {
    file: File,
    metadata: Metadata
}

impl PgsFile {
    pub(crate) fn new(file: File) -> Result<Self> {
        Ok(PgsFile {
            metadata: file.metadata()?,
            file
        })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<()> {
        if self.file.stream_position()? + buffer.len() as u64 > self.metadata.len() {
            return Err(Error::File(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "end of file")));
        }
        self.file.read_exact(buffer)?;
        Ok(())
    }

    pub fn read_n_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        if self.file.stream_position()? + N as u64 > self.metadata.len() {
            return Err(Error::File(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "end of file")));
        }
        let mut buffer: [u8; N] = [0; N];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn is_eof(&mut self) -> Result<bool> {
        Ok(self.file.stream_position()? >= self.metadata.len())
    }
}
