use crate::processor::{HrtorProcessor, FileInfo};
use crate::processor::constants::CommandStatus;
use std::path::PathBuf;
use std::fs::read_to_string;

impl HrtorProcessor {
    pub fn open(&mut self, arguments: &str) -> anyhow::Result<CommandStatus> {
        let path: PathBuf = PathBuf::from(arguments);
        self.editing_file = read_fileinfo(path)?;

        Ok(CommandStatus::Continue)
    }
}

fn read_fileinfo(path: PathBuf) -> anyhow::Result<FileInfo> {
    let result: FileInfo = FileInfo {
        path: Some(path.clone()),
        context: read_to_string(path).unwrap_or_else(|_| {
            eprintln!("your file cannot find. create a new buffer to continue this process.");
            String::new()
        }),
    };

    Ok(result)
}
