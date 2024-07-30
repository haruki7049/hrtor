use std::error::Error;

pub struct FileInfo {
    pub path: String,
    pub context: String,
}

impl FileInfo {
    pub fn get_file_info(path: String) -> Result<FileInfo, Box<dyn Error>> {
        Ok(FileInfo {
            path: path.clone(),
            context: std::fs::read_to_string(&path).unwrap_or_else(|_| {
                eprintln!("Your file cannot find. Created a new buffer to continue this process.");
                String::new()
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_info() {
        let file_info = FileInfo::get_file_info("Cargo.toml".to_string()).unwrap();
        assert_eq!(file_info.path, "Cargo.toml");
        assert!(file_info.context.len() > 0);
    }
}
