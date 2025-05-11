use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;
use std::path::PathBuf;

impl HrtorProcessor {
    pub fn write(&self, _arguments: &str) -> CommandStatus {
        {
            let editing_file = self.editing_file.lock().unwrap();
            save_file(&editing_file.path, &editing_file.context);
        }

        CommandStatus::Continue
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
