use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    pub fn tutorial(&self, _arguments: &str) -> CommandStatus {
        println!("This command is a hrtor tutorial.");

        CommandStatus::Continue
    }
}
