use std::io::Read;

use crate::pgs_error::Result;

pub trait PgsSeek {
    fn seek(&mut self, to: usize) -> Result<usize>;
    fn pos(&mut self) -> Result<usize>;
    fn len(&self) -> Result<usize>;
    fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) > 0
    }
}

pub trait PgsRead: Read + PgsSeek {}