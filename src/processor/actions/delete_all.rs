use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn delete_all(&self, _arguments: &str) -> CommandStatus {
        {
            let mut editing_file = self.editing_file.lock().unwrap();
            editing_file.context.clear();
        }

        CommandStatus::Continue
    }
}
