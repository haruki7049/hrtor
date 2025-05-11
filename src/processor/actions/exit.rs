use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;

impl HrtorProcessor {
    /// # Exit action
    /// If you want to exit Hrtor by your action, then you can set the return value to `CommandStatus::Quit`.
    pub fn exit(&self, _arguments: &str) -> CommandStatus {
        CommandStatus::Quit
    }
}
