use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn exit(&self, _arguments: String) -> CommandStatus {
        CommandStatus::Quit
    }
}
