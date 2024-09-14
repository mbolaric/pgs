use std::{fs::File, path::Path};

use crate::{pgs_error::Result, Error, PgsFile};

/// A struct for handling the opening of files to create `PgsFile` instances.
///
/// Currently, this struct does not maintain any internal state but provides a method to open a file and return a
/// `PgsFile` instance.
#[derive(Debug, Default)]
pub struct PgsReader {

}

impl PgsReader {
    /// Opens a file and returns a `PgsFile` instance.
    ///
    /// This method checks if the specified file exists and then attempts to open it. If successful, it creates a new
    /// `PgsFile` instance using the opened file.
    ///
    /// # Arguments
    /// * `sup_file_path` - A string slice representing the path to the file to be opened.
    ///
    /// # Returns
    /// Returns a `Result` containing either a `PgsFile` instance on success or an `Error` if the file does not exist,
    /// cannot be opened, or if creating the `PgsFile` instance fails.
    ///
    /// # Errors
    /// * `Error::File` - If the file does not exist or if the file cannot be opened.
    /// * Any other `Error` arising from `PgsFile::new` or file operations.
    pub fn open(sup_file_path: &str) -> Result<PgsFile> {
        let path = Path::new(sup_file_path);
        if !path.exists() {
            return Err(Error::File(std::io::Error::new(std::io::ErrorKind::NotFound, "File not Exists")));
        }
        let file = File::open(sup_file_path)?;
        PgsFile::new(file)
    }
}