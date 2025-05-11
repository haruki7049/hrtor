use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn exit(&self, _arguments: &str) -> CommandStatus {
        CommandStatus::Quit
    }
}
