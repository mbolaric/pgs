use std::{fmt::Debug, io::Read};

use crate::{pgs_error::Result, PgsSeek};

pub trait ByteOrder: Default + Debug + Clone {
    fn read_u16(buf: &[u8]) -> Result<u16>;
    fn read_u24(buf: &[u8]) -> Result<u32>;
    fn read_u32(buf: &[u8]) -> Result<u32>;
}

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

pub trait ReadBytes: Read {
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf: [u8; 1] = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_u16<T: ByteOrder>(&mut self) -> Result<u16> {
        let mut buf: [u8; 2] = [0; 2];
        self.read_exact(&mut buf)?;
        T::read_u16(&buf)
    }

    #[inline]
    fn read_u24<T: ByteOrder>(&mut self) -> Result<u32> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf)?;
        T::read_u24(&buf)
    }

    #[inline]
    fn read_u32<T: ByteOrder>(&mut self) -> Result<u32> {
        let mut buffer: [u8; 4] = [0; 4];
        self.read_exact(&mut buffer)?;
        T::read_u32(&buffer)
    }
}

impl<R: Read + ?Sized> ReadBytes for R {}

#[derive(Default)]
pub struct PgsMemoryBuffer {
    buffer: Vec<u8>,
    position: usize,
}

impl PgsMemoryBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
        }
    }

    pub fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut buffer: [u8; N] = [0; N];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }
    
    pub fn read_into_vec(&mut self, length: u32) -> Result<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![0; length as usize];
        let buffer_slice = buffer.as_mut_slice();
        self.read_exact(buffer_slice)?;
        Ok(buffer)
    }

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
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let len = Read::read(&mut self.remaining_slice(), buffer)?;
        self.position += len;
        Ok(len)
    }

    fn read_exact(&mut self, buffer: &mut [u8]) -> std::io::Result<()> {
        let buf_len = buffer.len();
        Read::read_exact(&mut self.remaining_slice(), buffer)?;
        self.position += buf_len;
        Ok(())
    }
}

impl From<Vec<u8>> for PgsMemoryBuffer {
    fn from(buffer: Vec<u8>) -> Self {
        PgsMemoryBuffer {
            buffer,
            position: 0,
        }
    }
}

impl From<&[u8]> for PgsMemoryBuffer {
    fn from(buffer: &[u8]) -> Self {
        PgsMemoryBuffer {
            buffer: Vec::from(buffer),
            position: 0,
        }
    }
}