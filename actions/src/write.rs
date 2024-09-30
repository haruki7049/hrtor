use crate::{CommandStatus, HrtorProcessor};

use super::command_status_ok;

impl HrtorProcessor {
    pub(crate) fn write(&self) -> CommandStatus {
        {
            let editing_file = self.editing_file.lock().unwrap();
            save_file(&editing_file.path, &editing_file.context);
        }
        command_status_ok()
    }
}

/// save file
fn save_file(filepath: &String, file_context: &String) {
    if let Err(err) = std::fs::write(filepath, file_context) {
        eprintln!("Error saving file: {}", err);
    } else {
        println!("file saved successfully");
    }
}
