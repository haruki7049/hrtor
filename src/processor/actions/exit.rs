use crate::processor::constants::CommandStatus;
use crate::processor::HrtorProcessor;

impl HrtorProcessor {
    pub fn exit(&self, _arguments: String) -> CommandStatus {
        CommandStatus::Quit
    }
}
