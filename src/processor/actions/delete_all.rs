use crate::processor::constants::CommandStatus;
use crate::processor::{HrtorProcessor, command_status_ok};

impl HrtorProcessor {
    pub fn delete_all(&self, _arguments: &str) -> CommandStatus {
        {
            let mut editing_file = self.editing_file.lock().unwrap();
            editing_file.context.clear();
        }
        command_status_ok()
    }
}
