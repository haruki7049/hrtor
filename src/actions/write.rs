use crate::{CommandStatus, Hrtor};

use super::result_ok;

impl Hrtor {
    pub(crate) fn write(&self) -> CommandStatus {
        save_file(&self.editing_file.path, &self.editing_file.context);
        result_ok()
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
