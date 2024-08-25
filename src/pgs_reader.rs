use std::{fs::File, path::Path};

use crate::{pgs_error::Result, Error, PgsFile};

#[derive(Debug, Default)]
pub struct PgsReader {

}

impl PgsReader {
    pub fn open(sup_file_path: &str) -> Result<PgsFile> {
        let path = Path::new(sup_file_path);
        if !path.exists() {
            return Err(Error::File(std::io::Error::new(std::io::ErrorKind::NotFound, "File not Exists")));
        }
        let file = File::open(sup_file_path)?;
        PgsFile::new(file)
    }
}