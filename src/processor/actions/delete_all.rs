use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn delete_all(&mut self, _arguments: &str) -> CommandStatus {
        {
            self.editing_file.context.clear();
        }

        CommandStatus::Continue
    }
}
