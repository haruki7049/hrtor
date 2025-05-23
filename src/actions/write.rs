use crate::ProcessorImplementation;
use anyhow::Context as _;
use hrtor_core::constants::CommandStatus;
use std::path::PathBuf;

pub trait HrtorWrite {
    fn write(&self, _arguments: &str) -> anyhow::Result<CommandStatus>;
}

impl HrtorWrite for ProcessorImplementation {
    fn write(&self, _arguments: &str) -> anyhow::Result<CommandStatus> {
        save_file(
            &self
                .buffer
                .path
                .clone()
                .context("OPEN ERROR: Cannot open the file because it is None")?,
            &self.buffer.context,
        );

        Ok(CommandStatus::Continue)
    }
}

/// save file
fn save_file(filepath: &PathBuf, file_context: &String) {
    if let Err(err) = std::fs::write(filepath, file_context) {
        eprintln!("Error saving file: {}", err);
    } else {
        println!("file saved successfully");
    }
}
