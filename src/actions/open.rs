use crate::ProcessorImplementation;
use hrtor_core::FileInfo;
use hrtor_core::constants::CommandStatus;
use std::fs::read_to_string;
use std::path::PathBuf;

pub trait HrtorOpen {
    fn open(&mut self, arguments: &str) -> anyhow::Result<CommandStatus>;
}

impl HrtorOpen for ProcessorImplementation {
    fn open(&mut self, arguments: &str) -> anyhow::Result<CommandStatus> {
        let path: PathBuf = PathBuf::from(arguments);
        self.buffer = read_fileinfo(path)?;

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
