use std::error::Error;

pub trait CommandLineArgsParser {
    /// Read a file and record into FileInfo struct
    fn read_fileinfo(&self) -> Result<FileInfo, Box<dyn Error>>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub path: String,
    pub context: String,
}
